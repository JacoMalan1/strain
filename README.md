# Strain

## ⚠️ Warning ⚠️

This project is currently in the **ALPHA** stage. There may be many bugs and it is not even guaranteed that it will compile/run at all.
That said, please report any bugs by filing an issue on the GitHub page. Please label your issues with the relevant labels.

In addition to the project being in **ALPHA**, the nature of this program is to create very taxing workloads on a processor.
Without proper cooling this could result in the processor reaching unsafe temperature. 
Please exercise caution and equip a CPU temperature reading program to make sure your CPU does not burn out.

A good CPU monitoring tool on GNU/Linux is [lm_sensors](https://wiki.archlinux.org/title/Lm_sensors). 


## Description

Strain is a lightweight CPU stress-testing utility written in Rust.
The main purpose it serves is to determine CPU stability and thermal output under load.

## Building

### Prerequisites

Ensure that the Rust compiler, the C compiler _(GCC)_, and Cargo are installed on your system.

Ensure that the following libraries are available on your system:

 - The GNU Multiple Precision library (`gmp`)
 - GMPFR (`gmpfr`)
 - MPC (`libmpc`)

Then, run the following command from the project root: `cargo build --frozen --release`.
This will produce a binary in the `target/release` folder called `strain`.

If the binary is not there, it is likely you have the `CARGO_TARGET_DIR` environment variable set.
In that case, the binary is located at `$CARGO_TARGET_DIR/release/strain`.
