//! The backend library which processes the ARINC files.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::pedantic,
    // clippy::missing_docs_in_private_items,
    clippy::unwrap_used
)]
// Glossary
// LSP = Loadable Software Parts
// MSP = Media Set Parts
mod error;
pub mod loads;
mod utils;
