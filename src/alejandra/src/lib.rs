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

#[cfg(any(
    all(
        target_arch = "aarch64",
        target_vendor = "unknown",
        target_os = "linux",
        target_env = "musl"
    ),
    all(
        any(target_arch = "armv6l", target_arch = "armv7l",),
        target_vendor = "unknown",
        target_os = "linux",
        target_env = "musleabihf"
    ),
    all(
        target_arch = "i686",
        target_vendor = "unknown",
        target_os = "linux",
        target_env = "musl"
    ),
    all(
        target_arch = "x86_64",
        target_vendor = "unknown",
        target_os = "linux",
        any(target_env = "gnu", target_env = "musl")
    ),
))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// Functions for formatting Nix code.
pub mod format;
pub(crate) mod formatter;
pub(crate) mod formatting_rules;
pub(crate) mod trivia;
pub(crate) mod utils;
/// Metadata.
pub mod version;
