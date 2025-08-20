# XDR serializetion and deserializetion library inspired by [serde-xdr](https://github.com/jvff/serde-xdr) with enum variant index support.

The overall `Data Type Map` are the same with [serde-xdr](https://github.com/jvff/serde-xdr).

Since `f128` is not in the stable rust, we do not support ser/deserialise it for now.

## Opaque type handling
`Vec<u8>` are handle as normal Vec<T>, this means every u8 element are serialized to be u32.
for XDR Opaque type, one should consider using `serde_bytes`, which provides `Bytes` and `BytesBuf`.

## Note on some non-XDR compatible type

some data type in serde data type model are not support by XDR spec (Map), we just leave a trivial implementation, user should keep this in mind that ser/deserializetion of those type are not widely accepted.

## Usage
```toml
[dependencies]
xdr-brk = "0.1"
serde = "1.0"
```

```rust
#[derive(Deserialize, Serialize)]
struct MyStruct {
    a: u32,
    b: String,
}

fn main() {
    let my_struct = MyStruct { a: 42, b: "Hello".to_string() };
    let serialized: Vec<u8> = xdr_brk::to_bytes(&my_struct).unwrap();
    let deserialized: MyStruct = xdr_brk::from_bytes(&serialized).unwrap();
    assert_eq!(my_struct, deserialized);
}
```
