fn main() -> Result<(), Box<dyn std::error::Error>> {
    buffa_build::Config::new()
        .files(&["../proto/example_oneof.proto"])
        .includes(&["../proto"])
        .generate_views(true)
        .generate_json(true)
        .extern_field_path_with_view(
            ".example_oneof.Msg.subpath",
            "::extern_brand::Foo",
            "::extern_brand::FooRef",
        )
        .extern_field_path(".example_oneof.Msg.index", "::extern_brand::Idx")
        .compile()?;
    Ok(())
}
