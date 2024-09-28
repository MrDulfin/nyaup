//! Serialize a Rust data structure into URL parameters string.

mod map;
mod seq;
mod simple;

use crate::error::Result;
use std::io;

/// A structure for serializing Rust values into URL parameters string.
pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W>
where
    W: io::Write,
{
    fn new(writer: W) -> Self {
        Serializer { writer }
    }
}

impl<'a, W> ::serde::ser::Serializer for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = crate::error::Error;

    type SerializeSeq = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = map::Serializer<'a, W>;
    type SerializeStruct = map::Serializer<'a, W>;
    type SerializeStructVariant = map::Serializer<'a, W>;

    #[inline]
    fn serialize_bool(self, _v: bool) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("bool"))
    }

    #[inline]
    fn serialize_i8(self, _v: i8) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("i8"))
    }

    #[inline]
    fn serialize_i16(self, _v: i16) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("i16"))
    }

    #[inline]
    fn serialize_i32(self, _v: i32) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("i32"))
    }

    #[inline]
    fn serialize_i64(self, _v: i64) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("i64"))
    }

    #[inline]
    fn serialize_u8(self, _v: u8) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("u8"))
    }

    #[inline]
    fn serialize_u16(self, _v: u16) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("u16"))
    }

    #[inline]
    fn serialize_u32(self, _v: u32) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("u32"))
    }

    #[inline]
    fn serialize_u64(self, _v: u64) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("u64"))
    }

    #[inline]
    fn serialize_f32(self, _v: f32) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("f32"))
    }

    #[inline]
    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("f64"))
    }

    #[inline]
    fn serialize_char(self, _v: char) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("char"))
    }

    #[inline]
    fn serialize_str(self, _v: &str) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("str"))
    }

    #[inline]
    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("bytes"))
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("unit struct"))
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Self::Error::UnsupportedAtTopLevel("unit variant"))
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Self::Error::UnsupportedAtTopLevel("sequence"))
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Self::Error::UnsupportedAtTopLevel("tuple"))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Self::Error::UnsupportedAtTopLevel("tuple struct"))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Self::Error::UnsupportedAtTopLevel("tuple variant"))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        write!(self.writer, "?")?;
        Ok(map::Serializer::new(&mut self.writer, true))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        write!(self.writer, "?")?;
        Ok(map::Serializer::new(&mut self.writer, false))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        write!(self.writer, "?")?;
        Ok(map::Serializer::new(&mut self.writer, false))
    }
}

/// Serialize the given data structure as URL parameters into the IO stream.
///
/// # Errors
///
/// Serialization fails if:
///
/// * `T`'s implementation of `Serialize` decides to fail,
/// * `T` is a type without keys, i.e. not a struct.
/// * `T` contains a nested struct,
/// * `T` contains a map.
#[inline]
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ::serde::ser::Serialize + ?Sized,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)?;
    Ok(())
}

/// Serialize the given data structure as a byte vector containing URL
/// parameters.
///
/// # Errors
///
/// Serialization fails if:
///
/// * `T`'s implementation of `Serialize` decides to fail,
/// * `T` is a type without keys, i.e. not a struct.
/// * `T` contains a nested struct,
/// * `T` contains a map.
#[inline]
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: ::serde::ser::Serialize + ?Sized,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given data structure as a String of URL parameters.
///
/// # Errors
///
/// Serialization fails if:
///
/// * `T`'s implementation of `Serialize` decides to fail,
/// * `T` is a type without keys, i.e. not a struct.
/// * `T` contains a nested struct,
/// * `T` contains a map.
#[inline]
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ::serde::ser::Serialize + ?Sized,
{
    let vec = to_vec(value)?;
    let string = String::from_utf8(vec)?;
    Ok(string)
}
