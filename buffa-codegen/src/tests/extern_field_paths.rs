//! Tests for the `extern_field_paths` codegen feature.

use super::*;

fn extern_path_config(entries: Vec<ExternFieldPath>) -> CodeGenConfig {
    CodeGenConfig {
        generate_views: false,
        extern_field_paths: entries,
        ..CodeGenConfig::default()
    }
}

#[test]
fn owned_string_field_uses_extern_path() {
    let mut file = proto3_file("ext.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![FieldDescriptorProto {
            name: Some("path".to_string()),
            number: Some(1),
            label: Some(Label::LABEL_OPTIONAL),
            r#type: Some(Type::TYPE_STRING),
            proto3_optional: Some(true),
            oneof_index: Some(0),
            ..Default::default()
        }],
        oneof_decl: vec![OneofDescriptorProto {
            name: Some("_path".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    let config = extern_path_config(vec![ExternFieldPath::new(
        ".Msg.path",
        "crate::wrap::Foo",
    )]);
    let files = generate(&[file], &["ext.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    assert!(
        content.contains("pub path: ::core::option::Option<crate::wrap::Foo>"),
        "owned struct field should use the extern path: {content}"
    );
}

#[test]
fn owned_u32_field_uses_extern_path() {
    let mut file = proto3_file("ext_u32.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![FieldDescriptorProto {
            name: Some("idx".to_string()),
            number: Some(1),
            label: Some(Label::LABEL_OPTIONAL),
            r#type: Some(Type::TYPE_UINT32),
            proto3_optional: Some(true),
            oneof_index: Some(0),
            ..Default::default()
        }],
        oneof_decl: vec![OneofDescriptorProto {
            name: Some("_idx".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    let config = extern_path_config(vec![ExternFieldPath::new(
        ".Msg.idx",
        "crate::wrap::Idx",
    )]);
    let files = generate(&[file], &["ext_u32.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    assert!(
        content.contains("pub idx: ::core::option::Option<crate::wrap::Idx>"),
        "owned u32 field should use the extern path: {content}"
    );
}

#[test]
fn repeated_string_field_uses_extern_path() {
    let mut file = proto3_file("ext_rep.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![make_field(
            "items",
            1,
            Label::LABEL_REPEATED,
            Type::TYPE_STRING,
        )],
        ..Default::default()
    });

    let config = extern_path_config(vec![ExternFieldPath::new(
        ".Msg.items",
        "crate::wrap::Item",
    )]);
    let files = generate(&[file], &["ext_rep.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    assert!(
        content.contains("pub items: ::buffa::alloc::vec::Vec<crate::wrap::Item>"),
        "repeated string field should use Vec<extern path>: {content}"
    );
}

/// Whitespace-normalized substring check.
///
/// `prettyplease` may split long expressions across multiple lines with
/// arbitrary indentation, so callers can't compare against a verbatim string.
/// This helper collapses every whitespace run in both the haystack and needle
/// to a single space (and additionally strips trailing commas inside angle
/// brackets that prettyplease inserts when wrapping a generic argument list).
fn contains_normalized(content: &str, needle: &str) -> bool {
    fn squash(s: &str) -> String {
        // Collapse whitespace, then drop the "trailing comma before `>`" that
        // prettyplease adds when it line-wraps `Trait<Arg,>` across multiple
        // lines. The replacement is iterated because nested generics produce
        // `Trait<Inner<Arg,>,>` shapes that need both commas dropped.
        let one_line = s.split_whitespace().collect::<Vec<_>>().join(" ");
        let mut prev = String::new();
        let mut cur = one_line;
        while prev != cur {
            prev = cur.clone();
            cur = cur
                .replace(", >", ">")
                .replace(",>", ">")
                .replace(", )", ")")
                .replace(",)", ")")
                .replace(", ]", "]")
                .replace(",]", "]")
                .replace(", <", "<")
                .replace(" >", ">")
                .replace(" )", ")")
                .replace("( ", "(")
                .replace("< ", "<");
        }
        cur
    }
    squash(content).contains(&squash(needle))
}

#[test]
fn owned_string_extern_uses_explicit_from_and_as_ref() {
    let mut file = proto3_file("ext_decode.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![FieldDescriptorProto {
            name: Some("path".to_string()),
            number: Some(1),
            label: Some(Label::LABEL_OPTIONAL),
            r#type: Some(Type::TYPE_STRING),
            proto3_optional: Some(true),
            oneof_index: Some(0),
            ..Default::default()
        }],
        oneof_decl: vec![OneofDescriptorProto {
            name: Some("_path".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    let config = extern_path_config(vec![ExternFieldPath::new(
        ".Msg.path",
        "crate::wrap::Foo",
    )]);
    let files = generate(&[file], &["ext_decode.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    assert!(
        contains_normalized(
            &content,
            "<crate::wrap::Foo as ::core::convert::From<\
             ::buffa::alloc::string::String>>::from"
        ),
        "decode site must use explicit-trait From form: {content}"
    );
    assert!(
        contains_normalized(
            &content,
            "<crate::wrap::Foo as ::core::convert::AsRef<\
             ::buffa::alloc::string::String>>::as_ref"
        ),
        "encode/size site must use explicit-trait AsRef form: {content}"
    );
}

#[test]
fn owned_numeric_extern_uses_explicit_from_and_as_ref() {
    let mut file = proto3_file("ext_numeric_decode.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![FieldDescriptorProto {
            name: Some("idx".to_string()),
            number: Some(1),
            label: Some(Label::LABEL_OPTIONAL),
            r#type: Some(Type::TYPE_UINT32),
            proto3_optional: Some(true),
            oneof_index: Some(0),
            ..Default::default()
        }],
        oneof_decl: vec![OneofDescriptorProto {
            name: Some("_idx".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    let config = extern_path_config(vec![ExternFieldPath::new(
        ".Msg.idx",
        "crate::wrap::Idx",
    )]);
    let files = generate(&[file], &["ext_numeric_decode.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    assert!(
        contains_normalized(&content, "<crate::wrap::Idx as ::core::convert::From<u32>>::from"),
        "numeric decode site must use explicit-trait From form: {content}"
    );
    assert!(
        contains_normalized(
            &content,
            "*<crate::wrap::Idx as ::core::convert::AsRef<u32>>::as_ref"
        ),
        "numeric encode site must use explicit-trait AsRef + deref: {content}"
    );
}

/// Implicit-presence numeric extern: the `is_non_default_expr` guard at
/// scalar size and write_to sites must use the unwrapped Inner value, not
/// `self.#ident` directly. Without this, `self.#ident != 0u32` would require
/// `Owned: PartialEq<u32>`, which the extern trait contract does not require.
#[test]
fn owned_implicit_presence_numeric_extern_wraps_default_check() {
    let mut file = proto3_file("ext_implicit_numeric.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![make_field(
            "idx",
            1,
            Label::LABEL_OPTIONAL,
            Type::TYPE_UINT32,
        )],
        ..Default::default()
    });

    let config = extern_path_config(vec![ExternFieldPath::new(
        ".Msg.idx",
        "crate::wrap::Idx",
    )]);
    let files = generate(
        &[file],
        &["ext_implicit_numeric.proto".to_string()],
        &config,
    )
    .expect("should generate");
    let content = joined(&files);

    // The non-default check must compare the wrapped *inner* value, not the
    // wrapper directly. Look for the AsRef-deref form on the LHS of the != .
    assert!(
        contains_normalized(
            &content,
            "*<crate::wrap::Idx as ::core::convert::AsRef<u32>>::as_ref(&self.idx) != 0u32"
        ),
        "implicit-presence numeric default check must wrap the value: {content}"
    );
}

#[test]
fn map_field_skips_extern_path() {
    let mut file = proto3_file("ext_map.proto");
    let map_entry = DescriptorProto {
        name: Some("StringsEntry".to_string()),
        field: vec![
            make_field("key", 1, Label::LABEL_OPTIONAL, Type::TYPE_STRING),
            make_field("value", 2, Label::LABEL_OPTIONAL, Type::TYPE_STRING),
        ],
        options: Some(MessageOptions {
            map_entry: Some(true),
            ..Default::default()
        })
        .into(),
        ..Default::default()
    };
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        nested_type: vec![map_entry],
        field: vec![FieldDescriptorProto {
            name: Some("strings".to_string()),
            number: Some(1),
            label: Some(Label::LABEL_REPEATED),
            r#type: Some(Type::TYPE_MESSAGE),
            type_name: Some(".Msg.StringsEntry".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    // Try to swap the map field — should be silently no-op.
    let config = extern_path_config(vec![ExternFieldPath::new(
        ".Msg.strings",
        "crate::wrap::ShouldNotAppear",
    )]);
    let files = generate(&[file], &["ext_map.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    assert!(
        !content.contains("ShouldNotAppear"),
        "extern path on a map field must be silently skipped: {content}"
    );
}

#[test]
fn bytes_fields_takes_precedence_over_extern_path() {
    let mut file = proto3_file("ext_bytes.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![make_field(
            "data",
            1,
            Label::LABEL_OPTIONAL,
            Type::TYPE_BYTES,
        )],
        ..Default::default()
    });

    let config = CodeGenConfig {
        generate_views: false,
        bytes_fields: vec![".".to_string()],
        extern_field_paths: vec![ExternFieldPath::new(
            ".Msg.data",
            "crate::wrap::ShouldNotAppear",
        )],
        ..CodeGenConfig::default()
    };
    let files = generate(&[file], &["ext_bytes.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    assert!(
        content.contains("::buffa::bytes::Bytes"),
        "bytes_fields should still apply: {content}"
    );
    assert!(
        !content.contains("ShouldNotAppear"),
        "extern path must lose to bytes_fields: {content}"
    );
}

#[test]
fn bytes_fields_takes_precedence_over_extern_path_repeated() {
    let mut file = proto3_file("ext_bytes_rep.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![make_field(
            "blobs",
            1,
            Label::LABEL_REPEATED,
            Type::TYPE_BYTES,
        )],
        ..Default::default()
    });

    let config = CodeGenConfig {
        generate_views: false,
        bytes_fields: vec![".".to_string()],
        extern_field_paths: vec![ExternFieldPath::new(
            ".Msg.blobs",
            "crate::wrap::ShouldNotAppear",
        )],
        ..CodeGenConfig::default()
    };
    let files = generate(&[file], &["ext_bytes_rep.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    assert!(
        content.contains("Vec<::buffa::bytes::Bytes>"),
        "repeated bytes should still be Vec<Bytes>: {content}"
    );
    assert!(
        !content.contains("ShouldNotAppear"),
        "extern path must lose to bytes_fields for repeated bytes too: {content}"
    );
}

#[test]
fn view_string_field_uses_view_extern_path() {
    let mut file = proto3_file("ext_view.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![make_field(
            "path",
            1,
            Label::LABEL_OPTIONAL,
            Type::TYPE_STRING,
        )],
        ..Default::default()
    });

    let config = CodeGenConfig {
        generate_views: true,
        extern_field_paths: vec![
            ExternFieldPath::new(".Msg.path", "crate::wrap::Foo")
                .with_view_path("crate::wrap::FooRef"),
        ],
        ..CodeGenConfig::default()
    };
    let files = generate(&[file], &["ext_view.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    // View struct field should use FooRef, not &'a str.
    assert!(
        content.contains("pub path: crate::wrap::FooRef"),
        "view field should use the view extern path: {content}"
    );
}

#[test]
fn view_string_field_without_view_path_uses_scalar() {
    let mut file = proto3_file("ext_view_noview.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![make_field(
            "path",
            1,
            Label::LABEL_OPTIONAL,
            Type::TYPE_STRING,
        )],
        ..Default::default()
    });

    let config = CodeGenConfig {
        generate_views: true,
        extern_field_paths: vec![ExternFieldPath::new(
            ".Msg.path",
            "crate::wrap::Foo",
        )],
        ..CodeGenConfig::default()
    };
    let files = generate(&[file], &["ext_view_noview.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    assert!(
        content.contains("pub path: &'a str"),
        "view field should fall back to &'a str when no view path: {content}"
    );
}

#[test]
fn view_extern_path_ignored_when_views_disabled() {
    let mut file = proto3_file("ext_view_off.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![make_field(
            "path",
            1,
            Label::LABEL_OPTIONAL,
            Type::TYPE_STRING,
        )],
        ..Default::default()
    });

    let config = CodeGenConfig {
        generate_views: false,
        extern_field_paths: vec![
            ExternFieldPath::new(".Msg.path", "crate::wrap::Foo")
                .with_view_path("crate::wrap::FooRef"),
        ],
        ..CodeGenConfig::default()
    };
    let files = generate(&[file], &["ext_view_off.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    assert!(
        !content.contains("FooRef"),
        "no view code should reference the view path: {content}"
    );
}

#[test]
fn view_string_decode_wraps_borrow_str_with_explicit_from() {
    let mut file = proto3_file("ext_view_decode.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![make_field(
            "path",
            1,
            Label::LABEL_OPTIONAL,
            Type::TYPE_STRING,
        )],
        ..Default::default()
    });

    let config = CodeGenConfig {
        generate_views: true,
        extern_field_paths: vec![
            ExternFieldPath::new(".Msg.path", "crate::wrap::Foo")
                .with_view_path("crate::wrap::FooRef"),
        ],
        ..CodeGenConfig::default()
    };
    let files = generate(&[file], &["ext_view_decode.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    assert!(
        contains_normalized(
            &content,
            "<crate::wrap::FooRef as ::core::convert::From<&str>>::from"
        ),
        "view decode site must use explicit-trait From form for the view extern: {content}"
    );
}

#[test]
fn numeric_field_with_view_path_is_build_error() {
    let mut file = proto3_file("ext_numeric_view.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![make_field(
            "idx",
            1,
            Label::LABEL_OPTIONAL,
            Type::TYPE_UINT32,
        )],
        ..Default::default()
    });

    let config = CodeGenConfig {
        generate_views: true,
        extern_field_paths: vec![
            ExternFieldPath::new(".Msg.idx", "crate::wrap::Idx")
                .with_view_path("crate::wrap::IdxRef"),
        ],
        ..CodeGenConfig::default()
    };
    let err = generate(&[file], &["ext_numeric_view.proto".to_string()], &config)
        .unwrap_err();

    let msg = format!("{err}");
    assert!(
        msg.contains("view_path") && msg.contains("numeric"),
        "error must explain the view_path-on-numeric mismatch: {msg}"
    );
}

#[test]
fn extern_field_path_on_bytes_is_build_error() {
    let mut file = proto3_file("ext_bytes_err.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![make_field(
            "data",
            1,
            Label::LABEL_OPTIONAL,
            Type::TYPE_BYTES,
        )],
        ..Default::default()
    });

    let config = extern_path_config(vec![ExternFieldPath::new(
        ".Msg.data",
        "crate::wrap::Data",
    )]);
    let err = generate(&[file], &["ext_bytes_err.proto".to_string()], &config)
        .unwrap_err();

    let msg = format!("{err}");
    assert!(
        msg.contains("extern_field_path") && msg.contains("TYPE_BYTES"),
        "error must explain the rejection: {msg}"
    );
}

#[test]
fn extern_path_on_oneof_variant_is_silently_skipped() {
    let mut file = proto3_file("ext_oneof.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![FieldDescriptorProto {
            name: Some("name".to_string()),
            number: Some(1),
            label: Some(Label::LABEL_OPTIONAL),
            r#type: Some(Type::TYPE_STRING),
            oneof_index: Some(0),
            // proto3_optional intentionally NOT set — this is a real oneof variant.
            ..Default::default()
        }],
        oneof_decl: vec![OneofDescriptorProto {
            name: Some("kind".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    let config = extern_path_config(vec![ExternFieldPath::new(
        ".Msg.name",
        "crate::wrap::ShouldNotAppear",
    )]);
    let files = generate(&[file], &["ext_oneof.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    assert!(
        !content.contains("ShouldNotAppear"),
        "extern path on a oneof variant must be silently skipped: {content}"
    );
}

#[test]
fn extern_field_path_on_message_is_build_error() {
    let mut file = proto3_file("ext_msg_err.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Inner".to_string()),
        field: vec![],
        ..Default::default()
    });
    file.message_type.push(DescriptorProto {
        name: Some("Outer".to_string()),
        field: vec![FieldDescriptorProto {
            name: Some("inner".to_string()),
            number: Some(1),
            label: Some(Label::LABEL_OPTIONAL),
            r#type: Some(Type::TYPE_MESSAGE),
            type_name: Some(".Inner".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    let config = extern_path_config(vec![ExternFieldPath::new(
        ".Outer.inner",
        "crate::wrap::Inner",
    )]);
    let err = generate(&[file], &["ext_msg_err.proto".to_string()], &config)
        .unwrap_err();

    let msg = format!("{err}");
    assert!(
        msg.contains("extern_field_path") && msg.contains("TYPE_MESSAGE"),
        "error must explain the rejection: {msg}"
    );
}

#[test]
fn extern_field_path_preserves_wire_format_shape() {
    let mut file = proto3_file("ext_invariance.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![FieldDescriptorProto {
            name: Some("path".to_string()),
            number: Some(1),
            label: Some(Label::LABEL_OPTIONAL),
            r#type: Some(Type::TYPE_STRING),
            proto3_optional: Some(true),
            oneof_index: Some(0),
            ..Default::default()
        }],
        oneof_decl: vec![OneofDescriptorProto {
            name: Some("_path".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    let baseline = generate(
        &[file.clone()],
        &["ext_invariance.proto".to_string()],
        &CodeGenConfig {
            generate_views: false,
            ..CodeGenConfig::default()
        },
    )
    .expect("baseline should generate");
    let baseline_content = joined(&baseline);

    let swapped = generate(
        &[file],
        &["ext_invariance.proto".to_string()],
        &extern_path_config(vec![ExternFieldPath::new(
            ".Msg.path",
            "crate::wrap::Foo",
        )]),
    )
    .expect("swapped should generate");
    let swapped_content = joined(&swapped);

    // Substrings that MUST appear in BOTH outputs — the underlying wire-format
    // calls are unchanged. `decode_string` is excluded because the baseline's
    // explicit-presence path uses `merge_string` (in-place); only the swapped
    // path uses `decode_string + From`.
    for invariant in [
        "::buffa::types::encode_string",
        "::buffa::types::string_encoded_len",
        "::buffa::encoding::WireType::LengthDelimited",
    ] {
        assert!(
            baseline_content.contains(invariant),
            "baseline missing wire-format invariant `{invariant}`: {baseline_content}"
        );
        assert!(
            swapped_content.contains(invariant),
            "swap broke wire-format invariant `{invariant}`: {swapped_content}"
        );
    }

    // Substrings that MUST appear ONLY in the swapped output — the
    // explicit-trait wrap is the only divergence. Use contains_normalized
    // because prettyplease may line-wrap these long type expressions.
    for swap_marker in [
        "<crate::wrap::Foo as ::core::convert::From<",
        "<crate::wrap::Foo as ::core::convert::AsRef<",
    ] {
        assert!(
            !contains_normalized(&baseline_content, swap_marker),
            "baseline must not contain swap marker `{swap_marker}`"
        );
        assert!(
            contains_normalized(&swapped_content, swap_marker),
            "swapped output must contain swap marker `{swap_marker}`: {swapped_content}"
        );
    }
}

/// Text-format encode for an implicit-presence extern numeric must route the
/// `is_non_default` check and the `enc.write_*` operand through the wrap.
/// Without this the generated `self.idx != 0u32` would not compile against a
/// wrapper that doesn't impl `PartialEq<u32>`.
#[test]
fn text_format_implicit_presence_numeric_extern_wraps_value() {
    let mut file = proto3_file("ext_text_numeric.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![make_field(
            "idx",
            1,
            Label::LABEL_OPTIONAL,
            Type::TYPE_UINT32,
        )],
        ..Default::default()
    });

    let config = CodeGenConfig {
        generate_views: false,
        generate_text: true,
        extern_field_paths: vec![ExternFieldPath::new(".Msg.idx", "crate::wrap::Idx")],
        ..CodeGenConfig::default()
    };
    let files = generate(&[file], &["ext_text_numeric.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    // Both the encode operand and the != 0 guard must use the AsRef-deref form.
    assert!(
        contains_normalized(
            &content,
            "enc.write_u32(*<crate::wrap::Idx as ::core::convert::AsRef<u32>>::as_ref(&self.idx))"
        ),
        "text encode must wrap numeric extern operand: {content}"
    );
    assert!(
        contains_normalized(
            &content,
            "*<crate::wrap::Idx as ::core::convert::AsRef<u32>>::as_ref(&self.idx) != 0u32"
        ),
        "text non-default check must wrap numeric extern operand: {content}"
    );
}

/// Text-format encode/decode for an extern string field must route through the
/// explicit-trait wrap helpers — `is_empty()` and `enc.write_string` on the
/// AsRef result, and `From<String>` on decode.
#[test]
fn text_format_string_extern_wraps_encode_and_decode() {
    let mut file = proto3_file("ext_text_string.proto");
    file.message_type.push(DescriptorProto {
        name: Some("Msg".to_string()),
        field: vec![make_field(
            "path",
            1,
            Label::LABEL_OPTIONAL,
            Type::TYPE_STRING,
        )],
        ..Default::default()
    });

    let config = CodeGenConfig {
        generate_views: false,
        generate_text: true,
        extern_field_paths: vec![ExternFieldPath::new(".Msg.path", "crate::wrap::Foo")],
        ..CodeGenConfig::default()
    };
    let files = generate(&[file], &["ext_text_string.proto".to_string()], &config)
        .expect("should generate");
    let content = joined(&files);

    assert!(
        contains_normalized(
            &content,
            "enc.write_string(<crate::wrap::Foo as ::core::convert::AsRef<\
             ::buffa::alloc::string::String>>::as_ref(&self.path))"
        ),
        "text encode must wrap string extern operand: {content}"
    );
    assert!(
        contains_normalized(
            &content,
            "self.path = <crate::wrap::Foo as ::core::convert::From<\
             ::buffa::alloc::string::String>>::from(dec.read_string()?.into_owned())"
        ),
        "text decode must wrap string extern with From<String>: {content}"
    );
}
