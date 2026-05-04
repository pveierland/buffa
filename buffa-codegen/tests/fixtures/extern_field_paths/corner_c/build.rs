fn main() -> Result<(), Box<dyn std::error::Error>> {
    buffa_build::Config::new()
        .files(&["../proto/example.proto"])
        .includes(&["../proto"])
        .generate_views(false)
        .generate_json(true)
        .extern_field_path(".example.Msg.path", "::extern_brand::Foo")
        .extern_field_path(".example.Msg.idx", "::extern_brand::Idx")
        .extern_field_path(".example.Msg.opt_path", "::extern_brand::Foo")
        .extern_field_path(".example.Msg.opt_idx", "::extern_brand::Idx")
        .compile()?;
    Ok(())
}
