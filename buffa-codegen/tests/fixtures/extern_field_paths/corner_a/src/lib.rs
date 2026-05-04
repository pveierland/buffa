//! Corner A: views off, json off. Exercises owned-side encode/decode/clear
//! through the brand contract.

#![allow(unused_imports, dead_code)]

#[allow(clippy::derivable_impls, clippy::match_single_binding)]
pub mod example {
    buffa::include_proto!("example");
}

#[cfg(test)]
mod tests {
    use super::example::Msg;
    use buffa::Message as _;
    use extern_brand::{Foo, Idx};

    #[test]
    fn owned_round_trips_through_extern_brand() {
        let original = Msg {
            path: Foo::from(String::from("hello")),
            idx: Idx::from(7),
            opt_path: Some(Foo::from(String::from("world"))),
            opt_idx: Some(Idx::from(13)),
            ..Default::default()
        };
        let bytes = original.encode_to_vec();
        let decoded = Msg::decode_from_slice(&bytes).expect("decode");
        assert_eq!(original, decoded);
    }

    #[test]
    fn clear_resets_extern_fields_to_brand_default() {
        let mut msg = Msg {
            path: Foo::from(String::from("not empty")),
            idx: Idx::from(42),
            opt_path: Some(Foo::from(String::from("filled"))),
            opt_idx: Some(Idx::from(99)),
            ..Default::default()
        };
        msg.clear();
        assert_eq!(msg.path, Foo::default());
        assert_eq!(msg.idx, Idx::default());
        assert!(msg.opt_path.is_none());
        assert!(msg.opt_idx.is_none());
    }
}
