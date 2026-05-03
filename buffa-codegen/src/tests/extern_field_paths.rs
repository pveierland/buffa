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
