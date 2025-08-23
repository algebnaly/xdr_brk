use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedLengthBytes<const N: usize> {
    bytes: [u8; N],
}

impl<const N: usize> FixedLengthBytes<N> {
    pub fn new(bytes: [u8; N]) -> Self {
        Self { bytes }
    }

    pub fn into_inner(self) -> [u8; N] {
        self.bytes
    }
}

impl<const N: usize> Deref for FixedLengthBytes<N> {
    type Target = [u8; N];
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl<const N: usize> DerefMut for FixedLengthBytes<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

impl<const N: usize> Serialize for FixedLengthBytes<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        crate::fixed_length_bytes::serialize(&self.bytes, serializer)
    }
}

impl<'de, const N: usize> Deserialize<'de> for FixedLengthBytes<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: [u8; N] = crate::fixed_length_bytes::deserialize(deserializer)?;
        Ok(Self { bytes })
    }
}

#[test]
fn test_fixed_length_bytes() {
    use crate::{from_bytes, to_bytes};
    let bytes = [1, 2, 3];
    let fixed_length_bytes = FixedLengthBytes::new(bytes);
    let serialized_bytes = to_bytes(&fixed_length_bytes).unwrap();
    assert_eq!(serialized_bytes, &[1, 2, 3, 0]);
    let deserialized_bytes: FixedLengthBytes<3> = from_bytes(&serialized_bytes).unwrap();
    assert_eq!(deserialized_bytes, fixed_length_bytes);
}
