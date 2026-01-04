pub mod java;
pub mod kotlin;
pub mod rust;

use crate::rule::parser::Visibility;

/// A missing doc violation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocViolation {
    pub line: usize,
    pub kind: DocKind,
    pub name: String,
}

/// Kind of item missing documentation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocKind {
    // Java types
    Class,
    Interface,
    Enum,
    Record,
    Annotation,
    Method,
    // Kotlin types (additional)
    Object,
    EnumClass,
    SealedClass,
    SealedInterface,
    DataClass,
    ValueClass,
    AnnotationClass,
    Typealias,
    Function,
    // Rust types (additional)
    Struct,
    Trait,
    TypeAlias,
    Union,
    Fn,
    MacroRules,
    Mod,
}

impl std::fmt::Display for DocKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocKind::Class => write!(f, "class"),
            DocKind::Interface => write!(f, "interface"),
            DocKind::Enum => write!(f, "enum"),
            DocKind::Record => write!(f, "record"),
            DocKind::Annotation => write!(f, "annotation"),
            DocKind::Method => write!(f, "method"),
            DocKind::Object => write!(f, "object"),
            DocKind::EnumClass => write!(f, "enum class"),
            DocKind::SealedClass => write!(f, "sealed class"),
            DocKind::SealedInterface => write!(f, "sealed interface"),
            DocKind::DataClass => write!(f, "data class"),
            DocKind::ValueClass => write!(f, "value class"),
            DocKind::AnnotationClass => write!(f, "annotation class"),
            DocKind::Typealias => write!(f, "typealias"),
            DocKind::Function => write!(f, "function"),
            DocKind::Struct => write!(f, "struct"),
            DocKind::Trait => write!(f, "trait"),
            DocKind::TypeAlias => write!(f, "type"),
            DocKind::Union => write!(f, "union"),
            DocKind::Fn => write!(f, "fn"),
            DocKind::MacroRules => write!(f, "macro_rules"),
            DocKind::Mod => write!(f, "mod"),
        }
    }
}

/// Config for Java doc checks
#[derive(Debug, Clone, Default)]
pub struct JavaDocConfig {
    pub class: Option<Visibility>,
    pub interface: Option<Visibility>,
    pub enum_: Option<Visibility>,
    pub record: Option<Visibility>,
    pub annotation: Option<Visibility>,
    pub method: Option<Visibility>,
}

/// Config for Kotlin doc checks
#[derive(Debug, Clone, Default)]
pub struct KotlinDocConfig {
    pub class: Option<Visibility>,
    pub interface: Option<Visibility>,
    pub object: Option<Visibility>,
    pub enum_class: Option<Visibility>,
    pub sealed_class: Option<Visibility>,
    pub sealed_interface: Option<Visibility>,
    pub data_class: Option<Visibility>,
    pub value_class: Option<Visibility>,
    pub annotation_class: Option<Visibility>,
    pub typealias: Option<Visibility>,
    pub function: Option<Visibility>,
}

/// Config for Rust doc checks
#[derive(Debug, Clone, Default)]
pub struct RustDocConfig {
    pub struct_: Option<Visibility>,
    pub enum_: Option<Visibility>,
    pub trait_: Option<Visibility>,
    pub type_alias: Option<Visibility>,
    pub union: Option<Visibility>,
    pub fn_: Option<Visibility>,
    pub macro_rules: Option<Visibility>,
    pub mod_: Option<Visibility>,
}
