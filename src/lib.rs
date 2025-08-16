mod de;
mod error;
mod ser;

pub use de::{XDRDeserializer, from_bytes};
pub use error::{Error, Result};
pub use ser::{Serializer, to_bytes};

pub(crate) fn padding_len(len: usize) -> usize {
    (4 - (len % 4)) % 4
}
pub(crate) const PADDING_BYTES: [u8; 3] = [0; 3];

pub const U32_SIZE: usize = 4;
pub const U64_SIZE: usize = 8;

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;

    #[test]
    fn serialize_void() {
        #[derive(Serialize)]
        struct Void;
    }
}
