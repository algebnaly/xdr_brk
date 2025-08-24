# XDR serializetion and deserializetion library

This library is inspired by [serde-xdr](https://github.com/jvff/serde-xdr) and adds support for Enum variants with explicit discriminants (manual enum tag values).

# Data Type Mapping


| Rust / Serde type       | XDR type            | Notes |
|--------------------------|---------------------------|-------|
| `i8`                     | `int`                 |  deserialize a value outside `i8` range will cause error |
| `i16`                    | `int`                 |  deserialize a value outside `i16` range will cause error |
| `i32`                    | `int`                 |   |
| `i64`                    | `hyper`               |   |
| `u8`                     | `unsigned int`        | deserialize a value outside `u8` range will cause error |
| `u16`                    | `unsigned int`        | deserialize a value outside `u16` range will cause error |
| `u32`                    | `unsigned int`        |   |
| `u64`                    | `unsigned hyper`      |   |
| `f32`                    | `float`               |   |
| `f64`                    | `double`              |   |
| `bool`                   | `bool`                |   |
| `String`                 | `string`              | XDR string is a length-prefixed byte sequence, intended for ASCII but often used with UTF-8 in modern implementations |
| see Opaque type handling | `opaque[n]`           | fixed-size byte array |
| see Opaque type handling | `opaque<>`            | variable-size byte array |
| `struct`                 | `struct`              |  |
| `enum` + `#[repr(u32)]`  | `enum`                | discriminant must be a u32 |
| `Option<T>`              | `optional<T>`         | Optional type |
| `[T; n]`                 | `T[n]`                | Fixed-length array |
| `Vec<T>`                 | `T<>`                 | variable-length array with size header |

## Opaque type handling
`Vec<u8>` are handle as normal Vec<T>, this means every u8 element are serialized to be u32.
for XDR Opaque type, one should consider using `serde_bytes`, which provides `Bytes` and `BytesBuf`.

for fixed length bytes, we provide `xdr_brk::fixed_length_bytes`, the following code shows its usage:

```rust
#[derive(Serialize)]
struct FixedLengthBytes{
    #[serde(with = "xdr_brk::fixed_length_bytes")]
    data: [u8; 16]
}
```

## Note on some non-XDR compatible type

some data type in serde data type model are not support by XDR spec (Map), we just leave a trivial implementation, user should keep this in mind that ser/deserializetion of those type are not widely accepted.

also note that Map in this crate are serialized as Vec<(Key, Value)>, and sorted by the binary representation of the key.

## Usage
```toml
[dependencies]
xdr_brk = "0.1"
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

if manually assign enum discriminants is wanted, the following code can be used:
```rust
use xdr_brk::{XDREnumSerialize,XDREnumDeserialize};

const fn discriminant_42() -> u32 {
    42
}

const C_4: u32 = 4;

#[repr(u32)]// Required for XDR enum discriminants
#[derive(Debug, PartialEq, XDREnumSerialize,XDREnumDeserialize)]
enum MyEnum {
    Variant1 = discriminant_42(),
    Variant2(u8, u16), // discriminant for this variant is 43
    Variant3(u8) = 100,
    Variant4{a: u32, b: u64} = C_4,
    #[default_arm] // handle unknown discriminant, see docs for details
    DefaultArm(u32),
}

fn main(){
    let my_enum = MyEnum::Variant2(1, 2);
    let serialized: Vec<u8> = xdr_brk::to_bytes(&my_enum).unwrap();
    let deserialized: MyEnum = xdr_brk::from_bytes(&serialized).unwrap();
    assert_eq!(my_enum, deserialized);
}
```
