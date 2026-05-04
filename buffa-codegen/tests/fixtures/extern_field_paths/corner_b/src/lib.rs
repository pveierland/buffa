//! Corner B: views on, json off. Exercises owned encode/decode + view
//! decode + view→owned conversion through the brand contract.

#![allow(unused_imports, dead_code)]

#[allow(clippy::derivable_impls, clippy::match_single_binding)]
pub mod example {
    buffa::include_proto!("example");
}

#[cfg(test)]
mod tests {
    use super::example::{__buffa::view::MsgView, Msg};
    use buffa::{Message as _, MessageView as _};
    use extern_brand::{Foo, Idx};

    fn sample() -> Msg {
        Msg {
            path: Foo::from(String::from("hello")),
            idx: Idx::from(7),
            opt_path: Some(Foo::from(String::from("world"))),
            opt_idx: Some(Idx::from(13)),
            ..Default::default()
        }
    }

    #[test]
    fn owned_round_trips_through_extern_brand() {
        let original = sample();
        let bytes = original.encode_to_vec();
        let decoded = Msg::decode_from_slice(&bytes).expect("decode");
        assert_eq!(original, decoded);
    }

    #[test]
    fn clear_resets_extern_fields_to_brand_default() {
        let mut msg = sample();
        msg.clear();
        assert_eq!(msg.path, Foo::default());
        assert_eq!(msg.idx, Idx::default());
        assert!(msg.opt_path.is_none());
        assert!(msg.opt_idx.is_none());
    }

    #[test]
    fn view_round_trip_via_owned() {
        let original = sample();
        let bytes = original.encode_to_vec();
        let view = MsgView::decode_view(&bytes).expect("decode_view");
        let owned: Msg = view.to_owned_message();
        assert_eq!(original, owned);
    }
}
