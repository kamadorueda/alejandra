//! Alejandra takes your Nix code and re-formats it in a consistent style.
//!
//! For more information please visit the
//! [Alejandra repository on GitHub](https://github.com/kamadorueda/alejandra).
#![deny(missing_docs)]
#![deny(rustdoc::bare_urls)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::private_doc_tests)]

pub(crate) mod builder;
pub(crate) mod children;
pub(crate) mod children2;
/// Functions for formatting Nix code.
pub mod format;
pub(crate) mod parsers;
pub(crate) mod position;
pub(crate) mod rules;
pub(crate) mod utils;
/// Metadata.
pub mod version;
