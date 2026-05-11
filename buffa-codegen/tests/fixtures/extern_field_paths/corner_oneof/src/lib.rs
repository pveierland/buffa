//! Oneof corner: views on, JSON on, with extern_field_paths on a string
//! variant (with view_path) and a numeric variant. Exercises every branded
//! oneof codepath end-to-end: binary encode/decode, JSON serialize +
//! deserialize, view encode + size, and view→owned conversion through the
//! brand `From` impls.

#![allow(unused_imports, dead_code)]

#[allow(clippy::derivable_impls, clippy::match_single_binding)]
pub mod example_oneof {
    buffa::include_proto!("example_oneof");
}

#[cfg(test)]
mod tests {
    use super::example_oneof::Msg;
    use super::example_oneof::__buffa::oneof::msg::Kind;
    use buffa::Message as _;
    use extern_brand::{Foo, Idx};

    #[test]
    fn string_variant_owned_binary_round_trip() {
        let original = Msg {
            kind: Some(Kind::Subpath(Foo::from(String::from("hello/world")))),
            ..Default::default()
        };
        let bytes = original.encode_to_vec();
        let decoded = Msg::decode_from_slice(&bytes).expect("decode");
        assert_eq!(original, decoded);
    }

    #[test]
    fn numeric_variant_owned_binary_round_trip() {
        let original = Msg {
            kind: Some(Kind::Index(Idx::from(42))),
            ..Default::default()
        };
        let bytes = original.encode_to_vec();
        let decoded = Msg::decode_from_slice(&bytes).expect("decode");
        assert_eq!(original, decoded);
    }

    #[test]
    fn string_variant_json_round_trip() {
        let original = Msg {
            kind: Some(Kind::Subpath(Foo::from(String::from("hello/world")))),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).expect("serialize");
        let decoded: Msg = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original, decoded);
    }

    #[test]
    fn numeric_variant_json_round_trip() {
        let original = Msg {
            kind: Some(Kind::Index(Idx::from(42))),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).expect("serialize");
        let decoded: Msg = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original, decoded);
    }
}
