# process-owned

This Rust crate provides easy access to multiple owners. Using the `ProcessOwned` struct, multiple owners can share the same data source. Internally, this uses an `Rc`, but the implementation will be modified for speed.

When paired with the `lazy_static` crate, this can be used to create a global data source that can be accessed from anywhere in the program.

## Example

```rust
use process_owned::ProcessOwned;

let mut data = ProcessOwned::new(5);
let mut data2 = data.clone();

assert_eq!(*data, 5);
assert_eq!(*data2, 5);

*data = 10;

assert_eq!(*data, 10);
assert_eq!(*data2, 10);
```

## License

This crate is licensed under the MIT license due to its extremely small size. See the `LICENSE` file for more information.