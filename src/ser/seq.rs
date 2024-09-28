use std::io;

pub struct Serializer<'a, W> {
    key: &'a str,
    writer: &'a mut W,
    first_param: bool,
}

impl<'a, W> Serializer<'a, W>
where
    W: io::Write,
{
    pub fn new(key: &'a str, writer: &'a mut W) -> Self {
        Serializer {
            key,
            writer,
            first_param: true,
        }
    }
}

impl<'a, W> ::serde::ser::SerializeSeq for Serializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = crate::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if !self.first_param {
            write!(self.writer, ",")?;
        }
        let simple = super::simple::Serializer::new_from_seq(self.first_param, self.key, &mut *self.writer);
        value.serialize(simple)?;
        if self.first_param {
            self.first_param = false;
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> ::serde::ser::SerializeTuple for Serializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = crate::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        ::serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> ::serde::ser::SerializeTupleStruct for Serializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = crate::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        ::serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> ::serde::ser::SerializeTupleVariant for Serializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = crate::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        ::serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
