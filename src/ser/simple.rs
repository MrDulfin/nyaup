//! Internal serializer for simple value

use std::io;

use serde::ser::SerializeSeq;

pub struct Serializer<'a, W> {
    first_param: bool,
    key: &'a str,
    writer: &'a mut W,
    sequence_allowed: bool,
}

impl<'a, W> Serializer<'a, W>
where
    W: io::Write,
{
    pub fn new_from_toplevel(first_param: bool, key: &'a str, writer: &'a mut W) -> Self {
        Serializer {
            first_param,
            key,
            writer,
            sequence_allowed: true,
        }
    }

    pub fn new_from_seq(first_param: bool, key: &'a str, writer: &'a mut W) -> Self {
        Serializer {
            first_param,
            key,
            writer,
            sequence_allowed: false,
        }
    }
}

impl<'a, W> ::serde::ser::Serializer for Serializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = crate::error::Error;

    type SerializeSeq = super::seq::Serializer<'a, W>;
    type SerializeTuple = super::seq::Serializer<'a, W>;
    type SerializeTupleStruct = super::seq::Serializer<'a, W>;
    type SerializeTupleVariant = super::seq::Serializer<'a, W>;

    type SerializeMap = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}={v}", self.key)?;
        } else {
            write!(self.writer, "{v}")?;
        }
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}={v}", self.key)?;
        } else {
            write!(self.writer, "{v}")?;
        }
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}={v}", self.key)?;
        } else {
            write!(self.writer, "{v}")?;
        }
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}={v}", self.key)?;
        } else {
            write!(self.writer, "{v}")?;
        }
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}={v}", self.key)?;
        } else {
            write!(self.writer, "{v}")?;
        }
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}={v}", self.key)?;
        } else {
            write!(self.writer, "{v}")?;
        }
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}={v}", self.key)?;
        } else {
            write!(self.writer, "{v}")?;
        }
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}={v}", self.key)?;
        } else {
            write!(self.writer, "{v}")?;
        }
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}={v}", self.key)?;
        } else {
            write!(self.writer, "{v}")?;
        }
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}={v}", self.key)?;
        } else {
            write!(self.writer, "{v}")?;
        }
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}={v}", self.key)?;
        } else {
            write!(self.writer, "{v}")?;
        }
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}={v}", self.key)?;
        } else {
            write!(self.writer, "{v}")?;
        }
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let v = String::from_iter(form_urlencoded::byte_serialize(v.as_bytes()));
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}={v}", self.key)?;
        } else {
            write!(self.writer, "{v}")?;
        }
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        if !self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            return Err(Self::Error::UnsupportedNestedStruct("bytes"));
        }
        let mut serializer = super::seq::Serializer::new(self.key, &mut *self.writer);
        for v in v {
            serializer.serialize_element(v)?;
        }
        serializer.end()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::UnsupportedNestedStruct("unit struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        use serde::Serialize;
        variant.serialize(self)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
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
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}=", self.key)?;
            Ok(super::seq::Serializer::new(&self.key, self.writer))
        } else {
            Err(Self::Error::UnsupportedNestedStruct("sequence"))
        }
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        if self.sequence_allowed {
            if !self.first_param {
                write!(self.writer, "&")?;
            }
            write!(self.writer, "{}=", self.key)?;
            Ok(super::seq::Serializer::new(self.key, self.writer))
        } else {
            Err(Self::Error::UnsupportedNestedStruct("sequence"))
        }
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Self::Error::UnsupportedNestedStruct("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Self::Error::UnsupportedNestedStruct("tuple variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Self::Error::UnsupportedNestedStruct("map"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Self::Error::UnsupportedNestedStruct("struct"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Self::Error::UnsupportedNestedStruct("struct variant"))
    }
}
