use serde::{Serialize, ser};

use crate::{
    PADDING_BYTES,
    error::{Error, Result},
    padding_len,
};

pub struct XDRSerializer {
    output: Vec<u8>,
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = XDRSerializer { output: Vec::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut XDRSerializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = MapSerializer<'a>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.extend((v as u32).to_be_bytes());
        Ok(())
    }
    fn serialize_i8(self, v: i8) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }
    fn serialize_i16(self, v: i16) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }
    fn serialize_i32(self, v: i32) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        Ok(())
    }
    fn serialize_i64(self, v: i64) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_u32(v as u32)
    }
    fn serialize_u16(self, v: u16) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_u32(v as u32)
    }
    fn serialize_u32(self, v: u32) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        Ok(())
    }
    fn serialize_f64(self, v: f64) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        Ok(())
    }

    fn serialize_char(self, v: char) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_u32(v as u32)
    }

    fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.extend((0 as u32).to_be_bytes());
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.output.extend((1 as u32).to_be_bytes());
        value.serialize(self)
    }

    /// do noting for unit
    fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_str(self, v: &str) -> std::result::Result<Self::Ok, Self::Error> {
        let len = v.len();
        let padding_len = padding_len(len);

        self.output.extend((len as u32).to_be_bytes());
        self.output.extend(v.as_bytes());
        self.output.extend(&PADDING_BYTES[..padding_len]);

        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        let bytes_len = v.len();
        if bytes_len > u32::MAX as usize {
            return Err(Error::Message("bytes too long".to_string()));
        }
        self.output.extend((bytes_len as u32).to_be_bytes());
        let padding_len = padding_len(bytes_len);
        self.output.extend(v);
        self.output.extend(&PADDING_BYTES[..padding_len]);
        Ok(())
    }

    /// do noting
    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }

    // note that, using serialize_*_variant will not handle manual assigned discriminants,
    // which can be handled by XDREnumSerialize derive macro
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        self.output.extend(variant_index.to_be_bytes());
        Ok(())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.output.extend(variant_index.to_be_bytes());
        value.serialize(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        self.output.extend(variant_index.to_be_bytes());
        Ok(self)
    }

    // just handle as enum
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        self.output.extend(variant_index.to_be_bytes());
        Ok(self)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        let len = len.ok_or(Self::Error::SequenceWithoutLength)? as u32;
        self.output.extend(len.to_be_bytes());
        Ok(self)
    }

    // there is no tuple in XDR, just handle as struct
    fn serialize_tuple(
        self,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    // there is no tuple struct in XDR, just handle as struct
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    // there is not map in XDR, just handle as Variable Length Array
    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        let len = len.ok_or(Self::Error::SequenceWithoutLength)? as u32;
        self.output.extend(len.to_be_bytes());
        Ok(MapSerializer {
            serializer: self,
            current_key: None,
            kv_pairs: Vec::new(),
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut XDRSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut XDRSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut XDRSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }
    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut XDRSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }
    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct MapSerializer<'a> {
    serializer: &'a mut XDRSerializer,
    current_key: Option<Vec<u8>>,
    kv_pairs: Vec<(Vec<u8>, Vec<u8>)>,
}

impl<'a> ser::SerializeMap for MapSerializer<'a> {
    type Ok = ();
    type Error = Error;
    fn serialize_key<T>(&mut self, key: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = XDRSerializer { output: Vec::new() };
        key.serialize(&mut serializer)?;
        if self.current_key.is_none() {
            self.current_key = Some(serializer.output);
        } else {
            return Err(Error::Message("previous key exists".to_owned()));
        }
        Ok(())
    }
    fn serialize_value<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = XDRSerializer { output: Vec::new() };
        value.serialize(&mut serializer)?;
        if let Some(key) = self.current_key.take() {
            self.kv_pairs.push((key, serializer.output));
        } else {
            return Err(Error::Message("no key exists".to_owned()));
        }
        Ok(())
    }
    fn end(mut self) -> std::result::Result<Self::Ok, Self::Error> {
        if self.current_key.is_some() {
            return Err(Error::Message("trailing key exists at end".to_owned()));
        }
        self.kv_pairs.sort_by(|a, b| a.0.cmp(&b.0));
        for (key, value) in self.kv_pairs {
            self.serializer.output.extend_from_slice(&key);
            self.serializer.output.extend_from_slice(&value);
        }
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut XDRSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_key<T>(&mut self, key: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut XDRSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }
    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut XDRSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }
    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use serde::Serialize;

    use crate::to_bytes;

    #[test]
    fn test_serialize_enum() {
        #[allow(unused)]
        #[derive(Debug, Serialize, PartialEq)]
        enum E {
            ZERO,
            ONE,
            TWO,
            THREE,
        }
        let e = E::TWO;
        let data = to_bytes(&e).unwrap(); // serialize enum to u32 (to_be_bytes)
        assert_eq!(data, vec![0, 0, 0, 2]);
    }

    #[test]
    fn test_serialize_bytes() {
        use serde_bytes::{ByteBuf, Bytes};
        let bytes = ByteBuf::from(vec![1, 2, 3]);
        let serialized_bytes = to_bytes(&bytes).unwrap();
        let expected_bytes = vec![0, 0, 0, 3, 1, 2, 3, 0];
        assert_eq!(&serialized_bytes, &expected_bytes);

        let data: &[u8] = &[1, 2, 3];
        let data_bytes = Bytes::new(data);
        let serialized_data_bytes = to_bytes(&data_bytes).unwrap();
        let expected_data_bytes = vec![0, 0, 0, 3, 1, 2, 3, 0];
        assert_eq!(serialized_data_bytes, expected_data_bytes);
    }

    #[test]
    fn test_serialize_map() {
        use std::collections::HashMap;
        let mut map: HashMap<u64, u16> = HashMap::new();
        map.insert(1, 2);
        map.insert(3, 4);
        let serialized_map_bytes = to_bytes(&map).unwrap();
        let expected_map_bytes = vec![
            0, 0, 0, 2, // len (u32)
            0, 0, 0, 0, 0, 0, 0, 1, // key 1 (u64)
            0, 0, 0, 2, // value 1 (u16)
            0, 0, 0, 0, 0, 0, 0, 3, // key 2 (u64)
            0, 0, 0, 4, // value 2 (u16)
        ];
        assert_eq!(serialized_map_bytes, expected_map_bytes);
    }

    #[test]
    fn test_serialize_fixed_length_array() {
        let data: [u8; 3] = [1, 2, 3];
        let serialized_data = to_bytes(&data).unwrap();
        let expected_data = vec![0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];
        assert_eq!(serialized_data, expected_data);
    }
}
