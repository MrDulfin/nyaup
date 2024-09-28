# Nyaup - Not Yet Another URL Params crate

### This is a fork of the [Yaup](https://github.com/meilisearch/yaup) crate, meant to solve my problem of yaup serializing `None`s as `null`. Currently the `tests::test_flattened_struct`, `tests::test_nested_unit`, `tests::test_sequence_as_key` tests do not pass, and I have no plans to fix them, so the examples may or may not work.

---

Serialize your structures as query parameters.
I made this crate because I didn't find anything that matched the structure of the query parameters used in Meilisearch.

Specificities of this query parameters format:
- The crate writes the initial `?` if there are parameters to send.
- You can only serialize structures that follow a "key-value" shape, like structures, `HashMap`, `BTreeMap`, etc.
- Sequences (arrays, vectors, tuples, etc) are comma-separated. `{ doggo: vec!["kefir", "echo"] }` serialize as `?doggo=kefir,echo`.
- Empty and `null` values are not ignored. `{ doggo: Vec::new(), catto: None }` serialize as `?doggo=&catto=null`.
- Return an error if you try to serialize a structure with multiple levels of key-value structures (i.e., an object containing a `HashMap` for example).

## Example

```rust
#[derive(Debug, serde::Serialize)]
enum Filter { New, Registered, Blocked }

#[derive(Debug, serde::Serialize)]
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
    yaup::to_string(&params).unwrap(),
    "?cursor=42&per_page=null&username=tamo&filter=New,Blocked"
);
```
## Thanks

This was originally a fork of [`serde_url_params`](https://github.com/boxdot/serde-url-params-rs) which is still maintained.
Thanks, `boxdot`, for the initial code.

Everything has been rewritten from scratch for the v0.3.0.

## License

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or
   <http://opensource.org/licenses/MIT>)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this document by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
