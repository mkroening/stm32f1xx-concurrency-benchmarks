# stm32f1xx-concurrency-benchmarks

Benchmarks of concurrency approaches on the STM32F1xx family of microcontrollers.

## Requirements

Your `rustc` needs to include [rust-lang/rust#69033](https://github.com/rust-lang/rust/pull/69033), so you need at least Rust `nightly-2020-03-22` or Rust 1.44.

Your Rust toolchain needs to support the `thumbv7m-none-eabi` target:

``` 
$ rustup target add thumbv7m-none-eabi
```

## Running the [benchmarks (`examples`)](examples)

The benchmarks toggle output pins around specific actions. The timings can be measured with an external logic analyzer, e.g. via [sigrok](https://sigrok.org/wiki/Main_Page) (see [scripts](scripts)).

### Requirements

* [OpenOCD](http://openocd.org/).

* [`arm-none-eabi-gdb`](https://www.gnu.org/software/gdb/)

### Adjusting to your hardware

The [memory region information](memory.x) included in this repository matches the Blue Pill (`STM32F103C8T6`).
You may need to adjust it according to your hardware.
For more information see [`cortex-m-quickstart`](https://github.com/rust-embedded/cortex-m-quickstart).

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

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `async-stm32f1xx` by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
