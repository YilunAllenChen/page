use crate::html_utils::make_tag;
use crate::models::Language;

impl Language {
    pub fn to_tag(&self) -> String {
        match self {
            Language::Haskell => make_tag("Haskell", "purple"),
            Language::Rust => make_tag("Rust", "orange"),
            Language::Python => make_tag("Python", "yellow"),
            Language::Go => make_tag("Go", "cyan"),
            Language::C => make_tag("C", "gray"),
            Language::OCaml => make_tag("OCaml", "blue"),
            Language::Cpp => make_tag("C++", "blue"),
            Language::Javascript => make_tag("Javascript", "yellow"),
            Language::Java => make_tag("Java", "red"),
        }
    }
}

// derive deserialize for Language
