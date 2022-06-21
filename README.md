# Yaup - Yet Another URL Params crate

This is a fork of [`serde_url_params`](https://github.com/boxdot/serde-url-params-rs).
I updated the way of serializing arrays:
Serializing `{ "food": ["baguette", "with", "cheese"] }`
- With `serde_url_params` returns `food=baguette&food=with&food=cheese`.
- With `yaup` it returns `food=baguette,with,cheese`.

And I got rids of the serialization of embedded structures.

## Example

```rust
#[derive(Debug, Serialize)]
enum Filter { New, Registered, Blocked }

#[derive(Debug, Serialize)]
struct Params {
    cursor: Option<usize>,
    per_page: Option<usize>,
    username: String,
    filter: Vec<Filter>,
}

let params = Params {
    cursor: Some(42),
    per_page: None,
    username: String::from("tamo"),
    filter: vec![Filter::New, Filter::Blocked],
};
assert_eq!(
    serde_url_params::to_string(&params).unwrap(),
    "cursor=42&username=boxdot&filter=New,Blocked"
);
```

## License

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this document by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
