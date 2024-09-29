use crate::error::Result;
use std::io;

pub struct Serializer<'a, W> {
    seq: bool,
    key: &'a str,
    writer: &'a mut W,
    first_param: bool,
}

impl<'a, W> Serializer<'a, W>
where
    W: io::Write,
{
    pub fn new(writer: &'a mut W, seq: bool) -> Self {
        Serializer {
            seq,
            key: "",
            writer,
            first_param: true,
        }
    }
}

impl<'a, W> ::serde::ser::SerializeMap for Serializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = crate::error::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        if !self.first_param {
            write!(self.writer, "&")?;
        }
        let simple = super::simple::Serializer::new_from_toplevel(false, &mut self.first_param, self.key, &mut *self.writer);
        key.serialize(simple)?;
        // write!(self.writer, "=")?;
        if self.first_param {
            self.first_param = false;
        }
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        let simple = super::simple::Serializer::new_from_toplevel(false, &mut self.first_param, &self.key, &mut *self.writer);
        value.serialize(simple)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> ::serde::ser::SerializeStruct for Serializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = crate::error::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ::serde::ser::Serialize,
    {
        let simple = super::simple::Serializer::new_from_toplevel(true, &mut self.first_param, key, &mut *self.writer);
        value.serialize(simple)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> ::serde::ser::SerializeStructVariant for Serializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = crate::error::Error;

    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        ::serde::ser::SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
