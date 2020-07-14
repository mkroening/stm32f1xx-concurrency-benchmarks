# stm32f1xx-concurrency-benchmarks

Benchmarks of concurrency approaches on the STM32F1xx family of microcontrollers.

## Requirements

This project overrides the Rust toolchain in [rust-toolchain](rust-toolchain), to make the benchmarks reproducible.
This Rust toolchain needs to support the `thumbv7m-none-eabi` target. Run this command in the project's folder:

``` 
$ rustup target add thumbv7m-none-eabi
```

## Hardware

Originally the benchmarks were run on an [STM32F103C8T6 — Blue Pill](https://stm32-base.org/boards/STM32F103C8T6-Blue-Pill).
For running the benchmarks on other microcontrollers, you might need to adjust the [memory region information](memory.x).

## Running the [benchmarks (`examples`)](examples)

The benchmarks toggle output pins which are to be measured externally.
The context switch benchmarks' output pin is `PA5`, while the latency benchmarks react to inputs to `PA5` and react on `PA6`.
To replicate the [measurements.tar.xz](https://git-ce.rwth-aachen.de/acs/internal/theses/bachelor/martin-kroening/stm32f1xx-concurrency-benchmarks/uploads/5622f8847243ff75903a62b182e69a8a/measurements.tar.xz), use a logic analyzers with a Cypress FX2 chip and measure with sigrok-cli using the provided [scripts](scripts).
To trigger reactions in the latency benchmarks, [tty_trigger.sh](scripts/tty_trigger.sh) was used.


### [cargo-embed](https://github.com/probe-rs/cargo-embed)

This project contains [Embed.toml](Embed.toml) which is configured for the Blue Pill and an ST-LINK/V2 programmer; Adjust it to your needs!

### [OpenOCD](http://openocd.org/)

Using OpenOCD requires [`arm-none-eabi-gdb`](https://www.gnu.org/software/gdb/).

### Starting the GDB Server

Start OpenOCD by replacing `$INTERFACE` with your debug probe (e.g. `stlink-v2.cfg`) and running:

``` 
$ openocd -f interface/$INTERFACE.cfg -f target/stm32f1x.cfg
```

### Running

You can now run the example via GDB:

``` 
$ cargo run --example <NAME> [--release]
```

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT) ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `stm32f1xx-concurrency-benchmarks` by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
