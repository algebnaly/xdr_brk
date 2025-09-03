use serde::Deserialize;
use serde::de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};

use crate::error::{Error, Result};
use crate::{U32_SIZE, U64_SIZE, padding_len};

#[derive(Debug)]
pub struct XDRDeserializer<'de> {
    pub(crate) input: &'de [u8],
}

impl<'de> XDRDeserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        XDRDeserializer { input }
    }
}

pub fn from_bytes<'a, T>(s: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = XDRDeserializer::from_bytes(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingBytes)
    }
}

impl<'de> XDRDeserializer<'de> {
    fn parse_bool(&mut self) -> Result<bool> {
        let v = u32::from_be_bytes(
            self.input
                .get(..U32_SIZE)
                .ok_or(Error::EndOfFile)?
                .try_into()
                .map_err(|_| Error::EndOfFile)?,
        );

        self.input = &self.input[U32_SIZE..];
        match v {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::Message(format!(
                "Option type descriminator should be 1 or 0 not {:?}",
                v
            ))),
        }
    }

    fn parse_u32(&mut self) -> Result<u32> {
        let v = u32::from_be_bytes(
            self.input
                .get(..U32_SIZE)
                .ok_or(Error::EndOfFile)?
                .try_into()
                .map_err(|_| Error::EndOfFile)?,
        );

        self.input = &self.input[U32_SIZE..];
        Ok(v)
    }

    fn parse_u64(&mut self) -> Result<u64> {
        let v = u64::from_be_bytes(
            self.input
                .get(..U64_SIZE)
                .ok_or(Error::EndOfFile)?
                .try_into()
                .map_err(|_| Error::EndOfFile)?,
        );

        self.input = &self.input[U64_SIZE..];
        Ok(v)
    }
    fn parse_i32(&mut self) -> Result<i32> {
        let v = i32::from_be_bytes(
            self.input
                .get(..U32_SIZE)
                .ok_or(Error::EndOfFile)?
                .try_into()
                .map_err(|_| Error::EndOfFile)?,
        );

        self.input = &self.input[U32_SIZE..];
        Ok(v)
    }

    fn parse_i64(&mut self) -> Result<i64> {
        let v = i64::from_be_bytes(
            self.input
                .get(..U64_SIZE)
                .ok_or(Error::EndOfFile)?
                .try_into()
                .map_err(|_| Error::EndOfFile)?,
        );

        self.input = &self.input[U64_SIZE..];
        Ok(v)
    }

    fn parse_f32(&mut self) -> Result<f32> {
        let v = f32::from_be_bytes(
            self.input
                .get(..U32_SIZE)
                .ok_or(Error::EndOfFile)?
                .try_into()
                .map_err(|_| Error::EndOfFile)?,
        );

        self.input = &self.input[U32_SIZE..];
        Ok(v)
    }

    fn parse_f64(&mut self) -> Result<f64> {
        let v = f64::from_be_bytes(
            self.input
                .get(..U64_SIZE)
                .ok_or(Error::EndOfFile)?
                .try_into()
                .map_err(|_| Error::EndOfFile)?,
        );

        self.input = &self.input[U64_SIZE..];
        Ok(v)
    }

    fn parse_bytes(&mut self) -> Result<Vec<u8>> {
        let len = self.parse_u32()?;
        let padded_len = len as usize + padding_len(len as usize);
        if self.input.len() < padded_len {
            return Err(Error::EndOfFile);
        }

        let v = self.input[..len as usize].to_vec();
        if self.input[len as usize..padded_len].iter().any(|&b| b != 0) {
            return Err(Error::NonZeroPadding);
        }
        self.input = &self.input[padded_len..];
        Ok(v)
    }

    fn parse_str(&mut self) -> Result<&'de str> {
        let len = self.parse_u32()?;
        let padded_len = len as usize + padding_len(len as usize);
        if self.input.len() < padded_len {
            return Err(Error::EndOfFile);
        }
        let v = &self.input[..len as usize];
        self.input = &self.input[padded_len..];
        let s = std::str::from_utf8(v)?;
        Ok(s)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut XDRDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::Message(
            "inner error, deserialize_any is not support".to_owned(),
        ))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let bytes = self.parse_bytes()?;
        visitor.visit_byte_buf(bytes)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_i32()?;
        let v = i8::try_from(v).map_err(|e| Self::Error::Message(format!("{:?}", e)))?;
        visitor.visit_i8(v)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_i32()?;
        let v = i16::try_from(v).map_err(|e| Self::Error::Message(format!("{:?}", e)))?;

        visitor.visit_i16(v)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_i64()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_u32()?;
        let v = u8::try_from(v).map_err(|e| Self::Error::Message(format!("{:?}", e)))?;

        visitor.visit_u8(v)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_u32()?;
        let v = u16::try_from(v).map_err(|e| Self::Error::Message(format!("{:?}", e)))?;

        visitor.visit_u16(v)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_u64()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parse_f32()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parse_f64()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_u32()?;
        let v = char::try_from(v).map_err(|e| Self::Error::Message(format!("{:?}", e)))?;

        visitor.visit_char(v)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let descriminator = self.parse_bool()?;
        if descriminator {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len = self.parse_u32()?;
        let value = visitor.visit_seq(LengthAccessor::new(self, len as usize))?;
        Ok(value)
    }
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(LengthAccessor::new(self, len))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(LengthAccessor::new(self, len))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.parse_u32()?;
        let map_accessor = LengthAccessor::new(self, len as usize);
        visitor.visit_map(map_accessor)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(LengthAccessor::new(self, fields.len()))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(MyEnumAccess::new(self))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct LengthAccessor<'a, 'de: 'a> {
    de: &'a mut XDRDeserializer<'de>,
    remain_items: usize,
}

impl<'a, 'de> LengthAccessor<'a, 'de> {
    fn new(de: &'a mut XDRDeserializer<'de>, count: usize) -> Self {
        Self {
            de,
            remain_items: count,
        }
    }
}

impl<'de, 'a> SeqAccess<'de> for LengthAccessor<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.remain_items == 0 {
            return Ok(None);
        }
        self.remain_items -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remain_items)
    }
}

impl<'a, 'de> MapAccess<'de> for LengthAccessor<'a, 'de> {
    type Error = Error;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.remain_items == 0 {
            return Ok(None);
        }
        self.remain_items -= 1;
        Ok(Some(seed.deserialize(&mut *self.de)?))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

struct MyEnumAccess<'a, 'de: 'a> {
    de: &'a mut XDRDeserializer<'de>,
}

impl<'a, 'de> MyEnumAccess<'a, 'de> {
    fn new(de: &'a mut XDRDeserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'a, 'de> EnumAccess<'de> for MyEnumAccess<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut self)?;
        Ok((val, self))
    }
}

impl<'de, 'a> de::Deserializer<'de> for &mut MyEnumAccess<'a, 'de> {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_any(visitor)
    }
    fn deserialize_bool<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_bool(visitor)
    }
    fn deserialize_bytes<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_bytes(visitor)
    }
    fn deserialize_byte_buf<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_byte_buf(visitor)
    }
    fn deserialize_char<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_char(visitor)
    }
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_enum(name, variants, visitor)
    }
    fn deserialize_f32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_f32(visitor)
    }
    fn deserialize_f64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_f64(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_i8(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_i16(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_i32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_i64(visitor)
    }

    fn deserialize_i128<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_i128(visitor)
    }

    /// the default enum Deserialize implementation of serde will call deserialize_identifier on a Deserializer
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        struct VarintVisitor {}
        impl<'de> Visitor<'de> for VarintVisitor {
            type Value = u32;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting u32")
            }
            fn visit_u32<E>(self, v: u32) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(v)
            }
        }
        let v = self.deserialize_u32(VarintVisitor {})?;
        visitor.visit_u32(v)
    }
    fn deserialize_ignored_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_ignored_any(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_map(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_newtype_struct(name, visitor)
    }
    fn deserialize_option<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_option(visitor)
    }
    fn deserialize_seq<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_seq(visitor)
    }
    fn deserialize_str<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_str(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_string(visitor)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_struct(name, fields, visitor)
    }

    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_tuple_struct(name, len, visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_u8(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_u16(visitor)
    }
    fn deserialize_u32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_u32(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_u64(visitor)
    }

    fn deserialize_u128<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_u128(visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_unit(visitor)
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.de.deserialize_unit_struct(name, visitor)
    }
}

impl<'de, 'a> VariantAccess<'de> for MyEnumAccess<'a, 'de> {
    type Error = Error;
    fn unit_variant(self) -> std::result::Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> std::result::Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(LengthAccessor::new(self.de, fields.len()))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(LengthAccessor::new(self.de, len))
    }
}

pub fn deserialize_len<'a, T: Deserialize<'a>>(data: &'a [u8]) -> Result<usize> {
    let total_len = data.len();
    let mut deserializer = XDRDeserializer::from_bytes(data);
    let _ = T::deserialize(&mut deserializer);
    let remaining_len = deserializer.input.len();
    Ok(total_len - remaining_len)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::from_bytes;

    #[test]
    fn test_deserialize_void_struct() {
        #[derive(Deserialize)]
        struct VoidStruct;
        let data: &[u8] = &[];
        let _void: VoidStruct = from_bytes(data).unwrap();
    }

    #[test]
    fn test_deserialize_u8() {
        let data: &[u8] = &[0, 0, 0, 42];
        let v: u8 = from_bytes(data).unwrap();
        assert_eq!(v, 42);
    }

    #[test]
    fn test_deserialize_u16() {
        let data: &[u8] = &[0, 0, 1, 1];
        let v: u16 = from_bytes(data).unwrap();
        assert_eq!(v, 257);
    }

    #[test]
    fn test_deserialize_u32() {
        let data: &[u8] = &[1, 2, 3, 4];
        let v: u32 = from_bytes(data).unwrap();
        assert_eq!(v, 16909060);
    }

    #[test]
    fn test_deserialize_simple_vec() {
        let data: &[u8] = &[0, 0, 0, 1, 0, 0, 0, 1];
        let v: Vec<u8> = from_bytes(data).unwrap();
        assert_eq!(v, vec![1]);
    }

    #[test]
    fn test_deserialize_simple_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct S {
            a: u16,
            b: String,
            c: f32,
        }

        let data: &[u8] = &[
            0, 0, 0, 42, // u16
            0, 0, 0, 5, // string_len
            b'h', b'e', b'l', b'l', b'o', 0, 0, 0, // string with padding
            63, 0, 0, 0,
        ];

        let s: S = from_bytes(data).unwrap();
        let expected_s = S {
            a: 42,
            b: "hello".to_owned(),
            c: 0.5,
        };
        assert_eq!(s, expected_s);
    }

    #[test]
    fn test_deserialize_nested_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct SA {
            a: u16,
            sb: SB,
            sc: SC,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct SB {
            b: String,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct SC {
            c: f32,
            sd: SD,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct SD {
            d: i8,
        }

        let data: &[u8] = &[
            0, 0, 0, 42, // u16, (42), big-endian
            0, 0, 0, 5, // string_len, (5), big-endian
            b'h', b'e', b'l', b'l', b'o', 0, 0, 0, // string with padding, ("hello")
            63, 0, 0, 0, // f32, (0.5), big-endian
            255, 255, 255, 255, // i8, (-1)
        ];
        let sa: SA = from_bytes(data).unwrap();
        let expected_sa = SA {
            a: 42,
            sb: SB {
                b: "hello".to_owned(),
            },
            sc: SC {
                c: 0.5,
                sd: SD { d: -1 },
            },
        };
        assert_eq!(sa, expected_sa);
    }

    #[test]
    fn test_deserialize_simple_enum() {
        #[derive(Debug, Deserialize, PartialEq)]
        #[repr(u32)]
        enum E {
            ZERO,
            ONE,
            TWO,
            THREE,
        }
        let data: &[u8] = &[
            0, 0, 0, 2, // TWO
        ];
        let e: E = from_bytes(data).unwrap();
        let expected_e = E::TWO;
        assert_eq!(e, expected_e);
    }

    #[test]
    fn test_deserialize_bytes() {
        use serde_bytes::ByteBuf;
        let data: &[u8] = &[0, 0, 0, 3, 1, 2, 3, 0];
        let bytes: ByteBuf = from_bytes(data).unwrap();
        assert_eq!(bytes, ByteBuf::from(vec![1, 2, 3]));
    }

    #[test]
    fn test_deserialize_map() {
        use std::collections::HashMap;
        let map_bytes = vec![
            0, 0, 0, 2, // len (u32)
            0, 0, 0, 0, 0, 0, 0, 1, // key 1 (u64)
            0, 0, 0, 2, // value 1 (u16)
            0, 0, 0, 0, 0, 0, 0, 3, // key 2 (u64)
            0, 0, 0, 4, // value 2 (u16)
        ];
        let map: HashMap<u64, u16> = from_bytes(&map_bytes).unwrap();
        let expected_map = HashMap::from([(1, 2), (3, 4)]);
        assert_eq!(map, expected_map);
    }

    #[test]
    fn test_deserialize_len() {
        use crate::de::deserialize_len;
        let data: &[u8] = &[
            0, 0, 0, 3, // real data
            1, 2, 3, 4, // remaining data
        ];
        let len = deserialize_len::<u32>(data).unwrap();
        assert_eq!(len, 4);

        #[allow(unused)]
        #[derive(Debug, Deserialize)]
        struct MyStruct {
            s: String,
            i: i32,
        }

        let data: &[u8] = &[
            0, 0, 0, 5, // len (u32)
            b'h', b'e', b'l', b'l', b'o', 0, 0, 0, // padding to align 4-byte boundary
            0, 0, 0, 42, // (u32)
        ];
        let len = deserialize_len::<MyStruct>(data).unwrap();

        assert_eq!(len, 16);
    }
}
