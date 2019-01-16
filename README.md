![logo](logo/raylib-rust_256x256.png)

# raylib-rust

Rust bindings for [raylib](http://www.raylib.com/), a simple and easy-to-use library to learn videogames programming.

## Development status

**DEPRECATED: Please use the more feature-complete bindings [deltaphc/raylib-rs](https://github.com/deltaphc/raylib-rs) instead.**

The raw binding layer `raylib-sys` is fucntionally complete with an automatic build script tested on macOS, Windows and Linux. This downloads raylib binaries and statically links your project. The higher level `raylib` crate is incomplete.

## Dependencies

The following tools are required to build `raylib-sys`:

- `curl`
- `clang` - Windows binaries can be downloaded [here](https://releases.llvm.org/download.html)
- `pkg-config` - Only required for macOS and Linux

## Supported targets

The following targets are tested to work:

- `x86_64-apple-darwin`
- `x86_64-pc-windows-msvc`
- `x86_64-unknown-linux-gnu`

However, it _should_ also work on the following:

- `i686-apple-darwin`
- `i686-pc-windows-msvc`
- `i686-unknown-linux-gnu`

Future support will be added for the following:

- `i686-pc-windows-gnu`
- `x86_64-pc-windows-gnu`

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

- Thanks [raysan5](https://github.com/raysan5) for building an amazing library.
