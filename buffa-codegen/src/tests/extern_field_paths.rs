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
        // prettyplease adds when it line-wraps `Trait<Arg,>` across two lines.
        let one_line = s.split_whitespace().collect::<Vec<_>>().join(" ");
        one_line
            .replace(", >", ">")
            .replace(", <", "<")
            .replace(" >", ">")
            .replace("< ", "<")
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
