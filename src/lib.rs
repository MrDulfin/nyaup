#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

#[doc(inline)]
pub use self::error::{Error, Result};
#[doc(inline)]
pub use self::ser::{to_string, to_vec, to_writer, Serializer};

mod error;
mod ser;

#[cfg(test)]
mod tests {
    use super::to_string;
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    enum Selection {
        A,
        B,
    }

    #[derive(Debug, Serialize)]
    struct Request {
        id: String,
        filter: Vec<String>,
        option: Option<String>,
        optional_filter: Option<Vec<String>>,
        select: Selection,
        select2: Vec<Selection>,
        num: Option<usize>,
        results: Vec<::std::result::Result<&'static str, &'static str>>,
    }

    #[test]
    fn test() {
        let request = Request {
            id: String::from("some_id"),
            filter: vec![String::from("filter1"), String::from("filter2")],
            option: None,
            optional_filter: Some(vec![String::from("filter3")]),
            select: Selection::A,
            select2: vec![Selection::A, Selection::B],
            num: Some(42),
            results: vec![Ok("pass"), Err("fail")],
        };
        let get_params = to_string(&request);
        insta::assert_snapshot!(
            get_params.unwrap(),
            @"?id=some_id&filter=filter1,filter2&option=null&optional_filter=filter3&select=A&select2=A,B&num=42&results=pass,fail"
        );
    }

    #[test]
    fn test_newtype_struct() {
        #[derive(Debug, Serialize)]
        struct NewType(usize);
        #[derive(Debug, Serialize)]
        struct Params {
            field: NewType,
        }
        let params = Params { field: NewType(42) };
        let url_params = to_string(&params);
        assert!(url_params.is_ok());
        insta::assert_snapshot!(url_params.unwrap(), @"?field=42");
    }

    #[test]
    fn test_tuple() {
        #[derive(Debug, Serialize)]
        struct Params {
            field: (usize, &'static str, f32),
        }
        let params = Params {
            field: (42, "hello", 3.15),
        };
        let url_params = to_string(&params);
        assert!(url_params.is_ok());
        insta::assert_snapshot!(url_params.unwrap(), @"?field=42,hello,3.15");
    }

    #[test]
    fn test_tuple_struct() {
        #[derive(Debug, Serialize)]
        struct TupleStruct(usize, &'static str, f32);
        #[derive(Debug, Serialize)]
        struct Params {
            field: TupleStruct,
        }
        let params = Params {
            field: TupleStruct(42, "hello", 3.15),
        };
        let url_params = to_string(&params);
        insta::assert_snapshot!(url_params.unwrap_err(), @"Tried to serialize a tuple struct in place of a value. Only simple values are supported on the right-hand side of a parameter.");
    }

    #[test]
    fn test_struct() {
        #[derive(Debug, Serialize)]
        struct A {
            username: String,
        }
        #[derive(Debug, Serialize)]
        struct Params {
            field: A,
        }
        // top level struct is supported
        {
            let params = A {
                username: String::from("boxdot"),
            };
            let url_params = to_string(&params);
            assert!(url_params.is_ok());
            insta::assert_snapshot!(url_params.unwrap(), @"?username=boxdot");
        }
        // nested struct is not supported
        {
            let params = Params {
                field: A {
                    username: String::from("boxdot"),
                },
            };
            let url_params = to_string(&params);
            insta::assert_snapshot!(url_params.unwrap_err(), @"Tried to serialize a struct in place of a value. Only simple values are supported on the right-hand side of a parameter.");
        }
    }

    #[test]
    fn test_struct_variant() {
        #[derive(Debug, Serialize)]
        enum StructVariant {
            A { username: String },
        }
        #[derive(Debug, Serialize)]
        struct Params {
            field: StructVariant,
        }
        // top level struct variant is supported
        {
            let params = StructVariant::A {
                username: String::from("boxdot"),
            };
            let url_params = to_string(&params);
            assert!(url_params.is_ok());
            insta::assert_snapshot!(url_params.unwrap(), @"?username=boxdot");
        }
        // nested struct variant is not supported
        {
            let params = Params {
                field: StructVariant::A {
                    username: String::from("boxdot"),
                },
            };
            let url_params = to_string(&params);
            insta::assert_snapshot!(url_params.unwrap_err(), @"Tried to serialize a struct variant in place of a value. Only simple values are supported on the right-hand side of a parameter.");
        }
    }

    #[test]
    fn test_urlencoded() {
        #[derive(Debug, Serialize)]
        struct Params {
            field: String,
        }
        let params = Params {
            field: String::from("{some=weird&param}"),
        };
        let url_params = to_string(&params);
        assert!(url_params.is_ok());
        insta::assert_snapshot!(url_params.unwrap(), @"?field=%7Bsome%3Dweird%26param%7D");
    }

    #[test]
    fn test_flattened_struct() {
        #[derive(Serialize, Debug)]
        pub struct Complex {
            real: f64,
            imag: f64,
        }

        #[derive(Serialize, Debug)]
        pub struct Params {
            x: u64,
            #[serde(flatten)]
            z: Option<Complex>,
        }

        let params = Params {
            x: 1,
            z: Some(Complex {
                real: 0.0,
                imag: 1.0,
            }),
        };
        let url_params = to_string(&params);
        insta::assert_snapshot!(
            url_params.unwrap(),
            @"?x=1&real=0&imag=1"
        );
    }

    #[test]
    fn test_seq_of_struct() {
        #[derive(Serialize, Debug)]
        pub struct Complex {
            real: f64,
            imag: f64,
        }

        #[derive(Serialize, Debug)]
        #[serde(transparent)]
        pub struct Params {
            seq: Vec<Complex>,
        }

        let params = Params {
            seq: vec![
                Complex {
                    real: 0.0,
                    imag: 1.0,
                },
                Complex {
                    real: 1.0,
                    imag: 0.0,
                },
            ],
        };
        let url_params = to_string(&params);
        insta::assert_snapshot!(url_params.unwrap_err(), @"Tried to serialize a sequence at the top level. Only key-value shapes are supported at the top level of a query parameter.");
    }

    #[test]
    fn test_empty_seq() {
        #[derive(Debug, Serialize)]
        struct StructVariant {
            array: Vec<u8>,
        }
        // top level struct variant is supported
        let params = StructVariant { array: vec![] };
        let url_params = to_string(&params);
        insta::assert_snapshot!(url_params.unwrap(), @"?array=");

        #[derive(Debug, Serialize)]
        struct OtherStructVariant {
            null: (),
        }
        let params = OtherStructVariant { null: () };
        let url_params = to_string(&params);
        insta::assert_snapshot!(url_params.unwrap(), @"?null=");
    }

    #[test]
    fn test_empty_opt_seq_with_other_param() {
        // See https://github.com/meilisearch/yaup/issues/4
        #[derive(Debug, Serialize)]
        struct StructVariant {
            array: Option<Vec<u8>>,
            other: bool,
        }
        // top level struct variant is supported
        let params = StructVariant {
            array: Some(vec![]),
            other: true,
        };
        let url_params = to_string(&params);
        insta::assert_snapshot!(url_params.unwrap(), @"?array=&other=true");
    }

    #[test]
    fn test_nested_unit() {
        #[derive(Debug, Serialize)]
        struct StructVariant {
            string: Option<String>,
            number: Option<u8>,
            unit1: (),
            after_unit: String,
            unit2: (),
        }

        let params = StructVariant {
            string: Some("".to_string()),
            number: None,
            unit1: (),
            after_unit: "hello".to_string(),
            unit2: (),
        };
        let url_params = to_string(&params);
        insta::assert_snapshot!(url_params.unwrap(), @"?string=&number=null&unit1=&after_unit=hello&unit2=");
    }

    #[test]
    fn test_unit() {
        let url_params = to_string(&());
        insta::assert_snapshot!(url_params.unwrap(), @"");
    }

    #[test]
    fn test_nested_sequences() {
        let url_params = to_string(&maplit::hashmap! { "a" => vec![vec![1, 2], vec![3, 4]]});
        insta::assert_snapshot!(url_params.unwrap_err(), @"Tried to serialize a sequence in place of a value. Only simple values are supported on the right-hand side of a parameter.");
    }

    #[test]
    fn test_sequence_as_key() {
        let url_params =
            to_string(&maplit::hashmap! { vec![1, 2] => vec![1, 2], vec![0] => vec![0] });
        insta::assert_snapshot!(url_params.unwrap(), @"?1,2=1,2&0=0");
    }

    #[test]
    fn test_transparent_vec() {
        #[derive(Debug, Serialize)]
        #[serde(transparent)]
        struct Struct {
            transparent: Vec<usize>,
        }

        #[derive(Debug, Serialize)]
        struct TopLevelStruct {
            transparent: Struct,
            hello: bool,
        }

        let url_params = TopLevelStruct {
            transparent: Struct {
                transparent: vec![0, 1, 2],
            },
            hello: true,
        };

        let url_params = to_string(&url_params);
        insta::assert_snapshot!(url_params.unwrap(), @"?transparent=0,1,2&hello=true");
    }
}
