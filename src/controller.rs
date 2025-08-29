use anyhow::Result;
use esp_idf_svc::hal::gpio::{Gpio2, Gpio13, Level, Output, PinDriver, Pins};
use std::{
    sync::mpsc::{self},
    time::Duration,
};

use crate::Command;

const COUNTDOWN_DELAY_MS: u64 = 500;
const COUNTDOWN_INITIAL_DELAY_MS: u64 = 2000;

pub struct CrosswalkController<'a> {
    man_pin: PinDriver<'a, Gpio2, Output>,
    hand_pin: PinDriver<'a, Gpio13, Output>,
    count: u8,
    cmd_rx: mpsc::Receiver<Command>,
    /// saved when an interruptible delay is interrupted
    pending_cmd: Option<Command>,
}

#[derive(Debug)]
pub enum CrosswalkState {
    Off,
    Man,
    Hand,
}

impl<'a> CrosswalkController<'a> {
    pub fn new(pins: Pins, cmd_rx: mpsc::Receiver<Command>) -> Result<Self> {
        Ok(Self {
            man_pin: PinDriver::output(pins.gpio2)?,   // D9/Pin2
            hand_pin: PinDriver::output(pins.gpio13)?, // D7/Pin13
            count: 5,
            cmd_rx,
            pending_cmd: None,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        self.set(CrosswalkState::Off)?;
        self.pending_cmd = None;

        loop {
            let cmd = if let Some(cmd) = self.pending_cmd.take() {
                cmd
            } else {
                match self.cmd_rx.recv() {
                    Ok(cmd) => cmd,
                    Err(_) => {
                        log::warn!("Command channel disconnected. Stopping..");
                        break;
                    }
                }
            };

            self.exec(cmd)?;
        }

        Ok(())
    }

    fn set(&mut self, state: CrosswalkState) -> Result<()> {
        log::debug!("Setting to {state:#?}");

        use CrosswalkState::*;
        match state {
            Off => {
                self.hand_pin.set_level(Level::Low)?;
                self.man_pin.set_level(Level::Low)?;
            }
            Man => {
                self.hand_pin.set_level(Level::Low)?;
                self.man_pin.set_level(Level::High)?;
            }
            Hand => {
                self.hand_pin.set_level(Level::High)?;
                self.man_pin.set_level(Level::Low)?;
            }
        }
        Ok(())
    }

    fn countdown(&mut self) -> Result<()> {
        log::info!("Counting down with count: {}", self.count);

        self.set(CrosswalkState::Man)?;

        if self.int_delay(COUNTDOWN_INITIAL_DELAY_MS)? {
            return Ok(());
        }

        self.set(CrosswalkState::Off)?;

        for i in 0..self.count {
            log::debug!("step {} of {}", i + 1, self.count);

            self.set(CrosswalkState::Hand)?;

            if self.int_delay(COUNTDOWN_DELAY_MS)? {
                return Ok(());
            }

            self.set(CrosswalkState::Off)?;

            if self.int_delay(COUNTDOWN_DELAY_MS)? {
                return Ok(());
            }
        }

        self.set(CrosswalkState::Hand)?;

        Ok(())
    }

    /// returns true if interrupted
    fn int_delay(&mut self, delay_ms: u64) -> Result<bool> {
        match self.cmd_rx.recv_timeout(Duration::from_millis(delay_ms)) {
            Ok(cmd) => {
                // this will make run after countdown exits
                self.pending_cmd = Some(cmd);
                Ok(true)
            }
            Err(mpsc::RecvTimeoutError::Timeout) => Ok(false),
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                Err(anyhow::anyhow!("Channel disconnected"))
            }
        }
    }

    fn exec(&mut self, cmd: Command) -> Result<()> {
        match cmd {
            Command::Off => self.set(CrosswalkState::Off)?,
            Command::Hand => self.set(CrosswalkState::Hand)?,
            Command::Man => self.set(CrosswalkState::Man)?,
            Command::Countdown => self.countdown()?,
            Command::Count(count) => {
                log::info!("Setting count to: {count}");
                self.count = count;
            }
        }
        Ok(())
    }
}
