# Kernel

A barebones kernel written in Rust.

## Getting Started

Building the kernel requires 3 main things: `rust nightly with armv7a-none-eabi`, `arm-none-eabi-gcc`, and `qemu-system-arm`. You can run/build the kernel locally by:

1. Cloning the repository:

    ```bash
    git clone https://github.com/potln/kernel.git
    cd kernel
    ```

2. Installing QEMU for ARM:

    On Debian-based systems:

    ```bash
    sudo apt update
    sudo apt install qemu-system-arm
    ```

    On macOS using Homebrew:

    ```bash
    brew install qemu
    ```

    For other platforms, refer to the [QEMU documentation](https://www.qemu.org/documentation/).

3. Installing the Rust nightly toolchain:

    ```bash
    rustup install nightly
    rustup default nightly
    ```

4. Installing the `arm-none-eabi` toolchain:
    On Debian-based systems:

    ```bash
    sudo apt update
    sudo apt install gcc-arm-none-eabi
    ```

    On macOS using Homebrew:

    ```bash
    brew install arm-none-eabi-gcc
    ```

    For other platforms, refer to the [GNU Arm Embedded Toolchain download page](https://developer.arm.com/downloads/-/gnu-rm).

5. Installing the `armv7a-none-eabi` target with Rustup:

    ```bash
    rustup target add armv7a-none-eabi
    ```

6. And finally, building/running the project:

    ```bash
    cargo build
    ```

    ```bash
    cargo run
    ```
