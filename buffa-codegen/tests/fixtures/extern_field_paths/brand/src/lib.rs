//! Brand wrappers used by the extern_field_paths integration fixtures.
//!
//! Mirrors the contract documented on
//! `buffa_codegen::CodeGenConfig::extern_field_paths`. Owned-side string
//! brand impls `AsRef<String>` (it owns its inner `String`); view-side
//! brand impls `AsRef<str>` (it borrows). Numeric brand impls
//! `AsRef<u32>` and is `Copy`.

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Foo(pub String);

impl ::core::convert::From<String> for Foo {
    fn from(s: String) -> Self {
        Self(s)
    }
}
impl ::core::convert::From<&str> for Foo {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}
impl ::core::convert::AsRef<String> for Foo {
    fn as_ref(&self) -> &String {
        &self.0
    }
}
impl<'a> ::core::convert::From<&FooRef<'a>> for Foo {
    fn from(r: &FooRef<'a>) -> Self {
        Self(r.0.to_string())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct FooRef<'a>(pub &'a str);

impl<'a> ::core::convert::From<&'a str> for FooRef<'a> {
    fn from(s: &'a str) -> Self {
        Self(s)
    }
}
impl<'a> ::core::convert::AsRef<str> for FooRef<'a> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Idx(pub u32);

impl ::core::convert::From<u32> for Idx {
    fn from(v: u32) -> Self {
        Self(v)
    }
}
impl ::core::convert::AsRef<u32> for Idx {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}
