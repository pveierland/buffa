//! Corner D: views on, json on. Exercises owned encode/decode + view
//! decode + view→owned conversion + serde JSON round-trip simultaneously
//! through the brand contract.

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
    fn view_round_trip_via_owned() {
        let original = sample();
        let bytes = original.encode_to_vec();
        let view = MsgView::decode_view(&bytes).expect("decode_view");
        let owned: Msg = view.to_owned_message();
        assert_eq!(original, owned);
    }

    #[test]
    fn json_round_trips_through_extern_brand() {
        let original = sample();
        let json = serde_json::to_string(&original).expect("serialize");
        let decoded: Msg = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original, decoded);
    }

    #[test]
    fn json_skips_implicit_presence_defaults() {
        let msg = Msg::default();
        let json = serde_json::to_string(&msg).expect("serialize");
        assert!(
            !json.contains("\"path\""),
            "default implicit string should be omitted: {json}"
        );
        assert!(
            !json.contains("\"idx\""),
            "default implicit numeric should be omitted: {json}"
        );
        assert!(
            !json.contains("\"opt_path\"") && !json.contains("\"optPath\""),
            "unset optional string should be omitted: {json}"
        );
        assert!(
            !json.contains("\"opt_idx\"") && !json.contains("\"optIdx\""),
            "unset optional numeric should be omitted: {json}"
        );
    }
}
