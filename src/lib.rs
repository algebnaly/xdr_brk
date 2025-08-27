mod de;
mod error;
pub mod fixed_length_bytes;
mod opaque;
mod ser;

pub use de::{XDRDeserializer, from_bytes, deserialize_len};
pub use error::{Error, Result};
pub use opaque::FixedLengthBytes;
pub use ser::{XDRSerializer, to_bytes, serialize_len};
pub use xdr_brk_enum::{XDREnumDeserialize, XDREnumSerialize};

pub(crate) fn padding_len(len: usize) -> usize {
    (4 - (len % 4)) % 4
}
pub(crate) const PADDING_BYTES: [u8; 3] = [0; 3];

pub const U32_SIZE: usize = 4;
pub const U64_SIZE: usize = 8;

#[cfg(test)]
mod tests {
    use crate::de::from_bytes;
    use crate::ser::to_bytes;
    use serde::Serialize;
    #[test]
    fn serialize_void() {
        #[derive(Serialize)]
        struct Void;
        let void = Void {};
        let bytes = to_bytes(&void).unwrap();
        assert_eq!(bytes, &[]);
    }
    #[test]
    fn test_serialize_deserialize_map() {
        use std::collections::HashMap;
        let mut map: HashMap<u64, u16> = HashMap::new();
        map.insert(1, 2);
        map.insert(3, 4);
        let serialized_map_bytes = to_bytes(&map).unwrap();
        let deserialized_map: HashMap<u64, u16> = from_bytes(&serialized_map_bytes).unwrap();
        assert_eq!(deserialized_map, map);
    }

    #[test]
    fn test_serialize_deserialize_fixed_length_bytes() {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct FixedLengthBytes<const N: usize> {
            #[serde(with = "crate::fixed_length_bytes")]
            bytes: [u8; N],
        }
        let bytes: FixedLengthBytes<3> = FixedLengthBytes { bytes: [1, 2, 3] };
        let serialized_bytes = to_bytes(&bytes).unwrap();
        assert_eq!(serialized_bytes, &[1, 2, 3, 0]);
        let deserialized_bytes: FixedLengthBytes<3> = from_bytes(&serialized_bytes).unwrap();
        assert_eq!(deserialized_bytes, bytes);
    }

    #[test]
    fn test_serialize_deserialize_string() {
        let s = String::from("hello");
        let serialized_s = to_bytes(&s).unwrap();
        assert_eq!(
            serialized_s,
            &[
                0, 0, 0, 5, // length 5 (u32)
                104, 101, 108, 108, 111, 0, 0, 0
            ]
        );
        let deserialized_s: String = from_bytes(&serialized_s).unwrap();
        assert_eq!(deserialized_s, s);

        let str_val = "hello";
        let serialized_str = to_bytes(&str_val).unwrap();
        assert_eq!(
            serialized_str,
            &[
                0, 0, 0, 5, // length 5 (u32)
                104, 101, 108, 108, 111, 0, 0, 0
            ]
        );
        let deserialized_str: &str = from_bytes(&serialized_str).unwrap();
        assert_eq!(deserialized_str, str_val);
    }
}
