# esp32-crosswalk-rs

ESP32 Rust Std Project for Controlling a Crosswalk Sign over MQTT.

Subscribes to:
 - `crosswalk/state` with payload
    - `OFF` turns everything off
    - `MAN` just show man
    - `HAND` just show hand
    - `COUNTDOWN` perform crosswalk countdown sequence. Note that the sign must be "programmed", so numbers wont display for the first countdown. Using any other `/state` will reset the programming. Changes to `/count` requre re-programming.
 - `crosswalk/count`: payload `uint` - what to countdown from 


## Running

Environment:

```bash
cd .embuild/espressif/esp-idf/v5.3.3
./install.fish all
# run given export command
```

```bash
./build_send
```

## Environment

### 1. Install Rust

### 2. Install `espup`:

```bash
cargo install espup --locked
```
You can also directly download pre-compiled release binaries or use cargo-binstall.

#### 2a. Install Necessary Toolchains:

```bash
espup install
```

#### 2b. Setup Environment Variables

`espup` will create an export file that contains some environment variables required to build projects.

On Windows (`%USERPROFILE%\export-esp.ps1`)
  - There is **no need** to execute the file for Windows users. It is only created to show the modified environment variables.

On Unix-based systems (`$HOME/export-esp.sh`). There are different ways of sourcing the file:
- Source this file in every terminal:
   1. Source the export file: `. $HOME/export-esp.sh`

   This approach requires running the command in every new shell.
- Create an alias for executing the `export-esp.sh`:
   1. Copy and paste the following command to your shell’s profile (`.profile`, `.bashrc`, `.zprofile`, etc.): `alias get_esprs='. $HOME/export-esp.sh'`
   2. Refresh the configuration by restarting the terminal session or by running `source [path to profile]`, for example, `source ~/.bashrc`.

   This approach requires running the alias in every new shell.
- Add the environment variables to your shell profile directly:
   1. Add the content of `$HOME/export-esp.sh` to your shell’s profile: `cat $HOME/export-esp.sh >> [path to profile]`, for example, `cat $HOME/export-esp.sh >> ~/.bashrc`.
   2. Refresh the configuration by restarting the terminal session or by running `source [path to profile]`, for example, `source ~/.bashrc`.

   This approach **doesn't** require any sourcing. The `export-esp.sh` script will be sourced automatically in every shell.

#### What `espup` Installs

To enable support for Espressif targets, `espup` installs the following tools:

- Espressif Rust fork with support for Espressif targets
- `nightly` toolchain with support for `RISC-V` targets
- `LLVM` [fork][llvm-github-fork] with support for `Xtensa` targets
- [GCC toolchain][gcc-toolchain-github-fork] that links the final binary

The forked compiler can coexist with the standard Rust compiler, allowing both to be installed on your system. The forked compiler is invoked when using any of the available [overriding methods][rustup-overrides].

> ⚠️ **Note**: We are making efforts to upstream our forks
> 1. Changes in `LLVM` fork. Already in progress, see the status in this [tracking issue][llvm-github-fork-upstream issue].
> 2. Rust compiler forks. If `LLVM` changes are accepted, we will proceed with the Rust compiler changes.

If you run into an error, please, check the [Troubleshooting][troubleshooting] chapter.

[llvm-github-fork]: https://github.com/espressif/llvm-project
[gcc-toolchain-github-fork]: https://github.com/espressif/crosstool-NG/
[rustup-overrides]: https://rust-lang.github.io/rustup/overrides.html
[llvm-github-fork-upstream issue]: https://github.com/espressif/llvm-project/issues/4
[troubleshooting]: ../troubleshooting/index.md

### Other Installation Methods for `Xtensa` Targets

- Using [`rust-build`][rust-build] installation scripts. This was the recommended way in the past, but now the installation scripts are feature frozen, and all new features will only be included in `espup`. See the repository README for instructions.
- Building the Rust compiler with `Xtensa` support from source. This process is computationally expensive and can take one or more hours to complete depending on your system. It isn't recommended unless there is a major reason to go for this approach. Here is the repository to build it from source: [`esp-rs/rust` repository][esp-rs-rust].

[rust-build]: https://github.com/esp-rs/rust-build#download-installer-in-bash
[esp-rs-rust]: https://github.com/esp-rs/rust

### 3. Install `std` Development Requirements

Regardless of the target architecture, make sure you have the following required tools installed to build [`std`][rust-esp-book-overview-std] applications:

- ESP-IDF Prerequisites:
  - Windows: [`python`][python-website-download] and [`git`][git-website-download]
  - Linux: See [Linux ESP-IDF prerequisites][esp-idf-linux].
  - macOS: See [macOS ESP-IDF prerequisites][esp-idf-macos].
- [`ldproxy`][embuild-github-ldproxy] binary crate: A tool that forwards linker arguments to the actual linker that is also given as an argument to `ldproxy`. Install it by running:
    ```shell
    cargo install ldproxy
    ```

> ⚠️ **Note**: The `std` runtime uses [ESP-IDF][esp-idf-github] (Espressif IoT Development Framework) as hosted environment but, users don't need to install it. ESP-IDF is automatically downloaded and installed by [`esp-idf-sys`][esp-idf-sys-github], a crate that all `std` projects need to use, when building a `std` application.

[rust-esp-book-overview-std]: ../overview/using-the-standard-library.md
[python-website-download]: https://www.python.org/downloads/windows/
[git-website-download]: https://git-scm.com/downloads
[embuild-github-ldproxy]: https://github.com/esp-rs/embuild/tree/master/ldproxy
[esp-idf-sys-github]: https://github.com/esp-rs/esp-idf-sys
[esp-idf-github]: https://github.com/espressif/esp-idf
[esp-idf-linux]: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/linux-macos-setup.html#for-linux-users
[esp-idf-macos]: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/linux-macos-setup.html#for-macos-users

### 4. Install `espflash`

```bash
cargo install espflash
```
