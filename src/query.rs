use crate::errors::QueryError;
use serde::{Deserialize, Serialize};
use streaming_iterator::StreamingIterator;
use tree_sitter::Query;
use tree_sitter::QueryCursor;

#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub name: String,               // The function name
    pub path: String,               // The URL path
    pub method: Option<HttpMethod>, // The HTTP verb (get, post, etc.)
    pub file_path: String,
    pub has_test: bool, // Whether this endpoint has associated tests
}
impl Endpoint {
    fn validate(&self) -> Result<(), QueryError> {
        match &self.method {
            Some(HttpMethod::Unknown(m)) => Err(QueryError::InvalidMethod(m.clone())),
            None => Err(QueryError::InvalidMethod("Missing HTTP method".to_string())),
            _ => Ok(()),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Unknown(String),
}

impl HttpMethod {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "get" => Some(HttpMethod::Get),
            "post" => Some(HttpMethod::Post),
            "put" => Some(HttpMethod::Put),
            "delete" => Some(HttpMethod::Delete),
            "patch" => Some(HttpMethod::Patch),
            _ => Some(HttpMethod::Unknown(s.to_string())),
        }
    }
}

pub const DECORATOR_QUERY: &'static str = r#"
(decorated_definition 
    (decorator
        (call
            function: (attribute attribute: (identifier) @decorator.name)
            arguments: (argument_list (string) @decorator.args)
        )
    )
    (function_definition    
        name: (identifier) @function.name
        parameters: (parameters) @function.params)
)
"#;

// return each match so we can pass it
pub fn run_query(
    query: &Query,
    source_code: &str,
    tree: &tree_sitter::Tree,
    file_path: &str,
) -> Result<Vec<Endpoint>, QueryError> {
    let mut list_endpoints = Vec::new();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());
    while let Some(match_) = matches.next() {
        let mut endpoint = Endpoint {
            name: String::new(),
            path: String::new(),
            method: None, // Default value
            file_path: file_path.to_string(),
            has_test: false,
        };
        for capture in match_.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let text = capture
                .node
                .utf8_text(source_code.as_bytes())
                .map_err(|e| QueryError::CaptureNodeError(e))?
                .to_string();
            endpoint.file_path = file_path.to_string();
            match capture_name {
                "decorator.name" => {
                    endpoint.method = HttpMethod::from_str(&text);
                } // "put")
                "function.name" => endpoint.name = text, // "update_item"
                "decorator.args" => {
                    endpoint.path = text.trim_matches('"').trim_matches('\'').to_string()
                } // '("/items/{item_id}") but replace \ and () like this "(\"/\")" '
                _ => {}
            }
        }
        endpoint.validate()?;

        list_endpoints.push(endpoint);
    }
    Ok(list_endpoints)
}

//write tests to check invalid method
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_validation_fails_on_unknown_method() {
        // Create an endpoint with an unknown method
        let ep = Endpoint {
            name: "test_func".to_string(),
            path: "/test".to_string(),
            method: Some(HttpMethod::Unknown("fetch".to_string())),
            file_path: "test.py".to_string(),
            has_test: false,
        };

        // Assert that validate() returns an error
        assert!(ep.validate().is_err());
    }

    #[test]
    fn test_endpoint_validation_passes_on_valid_method() {
        let ep = Endpoint {
            name: "test_func".to_string(),
            path: "/test".to_string(),
            method: Some(HttpMethod::Get), // Valid!
            file_path: "test.py".to_string(),
            has_test: false,
        };

        assert!(ep.validate().is_ok());
    }
}
