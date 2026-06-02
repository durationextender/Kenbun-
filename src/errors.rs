use std::str::Utf8Error;
use thiserror::Error;
use tree_sitter::LanguageError;

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Failed to read directory: {0}")]
    ReadDirError(#[from] ignore::Error),
    #[error("Permission denied for file: {0}")]
    PermissionDenied(String),
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Failed to load grammar: {0}")]
    GrammarLoadError(#[from] LanguageError),
    #[error("Parsing failed: the tree could not be generated")]
    ParseError,
}

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Failed to capture node text: {0}")]
    CaptureNodeError(Utf8Error),
    #[error("Invalid HTTP method: {0:?}")]
    InvalidMethod(String),
}
