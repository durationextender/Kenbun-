use crate::errors::ParserError;
use tree_sitter::Parser;
pub fn parse_file(source_code: &str) -> Result<tree_sitter::Tree, ParserError> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_python::LANGUAGE.into())
        .map_err(ParserError::GrammarLoadError)?;

    let tree = parser
        .parse(&source_code, None)
        .ok_or(ParserError::ParseError)?;
    Ok(tree)
}
