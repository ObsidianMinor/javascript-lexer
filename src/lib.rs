//! Lexer module consumes text input and produces linear stream of tokens.
//!
//! Tokens are defined in [token](lexer::token)
// Grammar rules are in [div](lexer::div)
//
// Supporting macro in [macros](lexer::macros)
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    trivial_casts,
    unused_import_braces,
    unused_qualifications
)]
#![warn(variant_size_differences)]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate phf;

use std::str;

/// Defines tokens
pub mod token;
#[macro_use]
mod macros;
mod equivalence;
/// Defines error handling
pub mod error;
/// Module for efficient string representation
pub mod internship {
    extern crate internship;
    pub use internship::*;
}
mod identifier;
mod number;
mod state;
mod state_machine;
mod string;
use self::{state_machine::parse, token::*};

/// Lexer implmementation
#[derive(Debug, Copy, Clone)]
pub struct Lexer;

impl Lexer {
    /// Transform string to stream of tokens
    pub fn lex_tokens(s: &str) -> Result<Vec<Token>, error::Error> {
        let mut tokens = parse(s)?;
        tokens.push(Token::EOF);
        Ok(tokens)
    }
}
