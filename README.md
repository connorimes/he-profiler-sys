# Heartbeats/EnergyMon Profiler Rust Bindings

The `he-profiler-sys` crate provides declarations and linkage for the
`he-profiler` C library.
Following the *-sys package conventions, this crate does not define
higher-level abstractions over the native library functions.

The latest `he-profiler` C library can be found at
[https://github.com/he-profiler/he-profiler](https://github.com/he-profiler/he-profiler).

## Dependencies

In order to use the `he-profiler-sys` crate, you must have the
`he-profiler` library installed to the system where it can be found
by `pkg-config`.

## Usage
Add `he-profiler-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies.he-profiler-sys]
git = "https://github.com/he-profiler/he-profiler-sys.git"
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
