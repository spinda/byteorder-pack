# byteorder-pack [![Latest Version]][crates.io] [![Docs Status]][docs.rs]

[Latest Version]: https://img.shields.io/crates/v/byteorder-pack.svg
[crates.io]: https://crates.io/crates/byteorder-pack
[Docs Status]: https://docs.rs/byteorder-pack/badge.svg
[docs.rs]: https://docs.rs/byteorder-pack

A binary data reader and writer that is similar to Python's struct module,
but makes use of Rust's typing system.

## Example

```rust
use std::io::Cursor;
use byteorder_pack::UnpackFrom;

let mut cursor = Cursor::new(vec![0x01, 0x02, 0x00, 0x03, 0x00, 0x04]);

let (a, b, cd) = <(u8, u8, [u16; 2])>::unpack_from_be(&mut cursor).unwrap();

assert_eq!(a, 1);
assert_eq!(b, 2);
assert_eq!(cd, [3, 4]);
```