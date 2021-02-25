# fs-tracing

fs-tracing is a drop-in replacement for [`std::fs`](std::fs) that provides an auxiliary
information (such as paths) on error via [`tracing`](https://github.com/tokio-rs/tracing).

## Usage
You need to install [`tracing_error::ErrorLayer`](https://docs.rs/tracing-error/0.1.2/tracing_error/struct.ErrorLayer.html)
for capturing the error context. For example, the following function installs `ErrorLayer`.

```rust
// https://docs.rs/tracing-error/0.1.2/tracing_error/index.html
pub fn install() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;

    let subscriber = tracing_subscriber::Registry::default().with(ErrorLayer::default());

    tracing::subscriber::set_global_default(subscriber).unwrap();
}
```

For more information, please visit [https://docs.rs/tracing-subscriber/0.2.16/tracing_subscriber/registry/index.html](https://docs.rs/tracing-subscriber/0.2.16/tracing_subscriber/registry/index.html).

Then, you can replace `std::fs` with `fs_tracing` in your code and you get nice error messages.

## Errors
fs-tracing returns [`std::io::Error`](std::io::Error) on errors for compatibility, although
the returned error contains the context information such as the kind of the operation and the
values passed as arguments.

For example, when you open a file which does not exist, the error message returned by fs-tracing
prints the operation name (`fs_tracing::read`) and the offending path (`/not_exist`):
```
No such file or directory (os error 2)
Trace:
   0: fs_tracing::read
           with path="/not_exist"
             at src/lib.rs:652
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
