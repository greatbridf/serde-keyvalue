use serde::{ser::{Impossible, SerializeStruct}, Serializer};

/// A serializer that outputs key-value pairs in a string format.
/// The output is a single string where each key-value pair is separated by spaces,
/// and each key is followed by an equals sign and its corresponding value.
/// 
/// For example: "key1=value1 key2=value2 key3=value3".
/// 
/// This serializer is designed to be used with structs,
/// where each field is serialized as a key-value pair.
/// 
/// # Example
/// ```
/// use serde::Serialize;
/// use serde_keyvalue::KeyValueSerializer;
/// 
/// #[derive(Serialize)]
/// enum Color {
///     Red,
///     Blue,
/// }
/// 
/// #[derive(Serialize)]
/// struct MyStruct {
///     key1: String,
///     key2: i32,
///     key3: bool,
///     key4: f64,
///     key5: Color,
/// }
/// 
/// let my_struct = MyStruct {
///     key1: "value1".to_string(),
///     key2: 42,
///     key3: true,
///     key4: 1.5,
///     key5: Color::Red,
/// };
/// 
/// let mut serializer = KeyValueSerializer::new();
/// my_struct.serialize(&mut serializer).unwrap();
/// let output = serializer.into_output();
/// 
/// assert_eq!(output, "key1=value1 key2=42 key3=True key4=1.5 key5=Red");
/// ```
pub struct KeyValueSerializer {
    top_parsed: bool,
    output: String,
}

pub struct KeyValueSerializerCounted<'s>(&'s mut KeyValueSerializer, usize);

impl KeyValueSerializer {
    /// Creates a new `KeyValueSerializer` instance with an empty output string.
    pub fn new() -> Self {
        KeyValueSerializer {
            top_parsed: false,
            output: String::new(),
        }
    }

    /// Consumes the serializer and returns the serialized output as a string.
    pub fn into_output(self) -> String {
        self.output
    }
    
    fn serialize_signed(&mut self, v: i64) -> Result<(), std::fmt::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_unsigned(&mut self, v: u64) -> Result<(), std::fmt::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }
}

impl SerializeStruct for KeyValueSerializerCounted<'_> {
    type Ok = ();
    type Error = std::fmt::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        self.0.output.push_str(key);
        self.0.output.push('=');
        value.serialize(&mut *self.0)?;
        
        if self.1 > 1 {
            self.1 -= 1;
            self.0.output.push(' ');
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
    
    fn skip_field(&mut self, _: &'static str) -> Result<(), Self::Error> {
        self.1 -= 1;
        Ok(())
    }
}

impl<'a> Serializer for &'a mut KeyValueSerializer {
    type Ok = ();
    type Error = std::fmt::Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = KeyValueSerializerCounted<'a>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(if v { "True" } else { "False" });
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_signed(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_signed(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_signed(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_signed(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_unsigned(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_unsigned(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_unsigned(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_unsigned(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.output.push(v);
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(v);
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unreachable!("None should have been skipped in serialization")
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(variant);
        Ok(())
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        if !self.top_parsed {
            self.top_parsed = true;
            Ok(KeyValueSerializerCounted(self, len))
        } else {
            Err(std::fmt::Error)
        }
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}
