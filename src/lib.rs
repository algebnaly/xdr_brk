mod de;
mod error;
mod ser;

pub use de::{XDRDeserializer, from_bytes};
pub use error::{Error, Result};
pub use ser::{Serializer, to_bytes};
pub use xdr_brk_enum::{XDREnumDeserialize, XDREnumSerialize};

pub(crate) fn padding_len(len: usize) -> usize {
    (4 - (len % 4)) % 4
}
pub(crate) const PADDING_BYTES: [u8; 3] = [0; 3];

pub const U32_SIZE: usize = 4;
pub const U64_SIZE: usize = 8;

#[cfg(test)]
mod tests {
    use crate::ser::to_bytes;
    use crate::de::from_bytes;
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
}
