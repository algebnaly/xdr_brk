use serde::{Deserializer, Serializer, de, ser::SerializeTuple};
use std::fmt;

pub fn serialize<S, const N: usize>(bytes: &[u8; N], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let chunks = (N + 3) / 4; // number of u32 chunks needed
    let mut seq = serializer.serialize_tuple(chunks)?;
    for i in 0..chunks {
        let start = i * 4;
        let end = (start + 4).min(N);
        let mut buf = [0u8; 4];
        buf[..end - start].copy_from_slice(&bytes[start..end]);
        let val = u32::from_be_bytes(buf);
        seq.serialize_element(&val)?;
    }
    seq.end()
}

pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[u8; N], D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor<const N: usize>;

    impl<'de, const N: usize> de::Visitor<'de> for Visitor<N> {
        type Value = [u8; N];

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} fixed-length bytes", N)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut bytes = [0u8; N];
            let chunks = (N + 3) / 4;
            let mut idx = 0;

            for _ in 0..chunks {
                let val: u32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(idx, &self))?;
                let chunk = val.to_be_bytes();
                let len = (N - idx).min(4);
                bytes[idx..idx + len].copy_from_slice(&chunk[..len]);
                idx += len;
            }

            Ok(bytes)
        }
    }

    deserializer.deserialize_tuple((N + 3) / 4, Visitor::<N>)
}
