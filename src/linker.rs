use crate::errors::QueryError;
use streaming_iterator::StreamingIterator;
use tree_sitter::Query;
use tree_sitter::QueryCursor;

pub struct TestEndpoint {
    pub name: String,
    pub file_path: String,
}
pub const TEST_QUERY: &'static str = r#"
(function_definition
    name: (identifier) @test.name
    (#match? @test.name "^test_.*")
)
"#;

// return each match so we can pass it
pub fn run_test_query(
    query: &Query,
    source_code: &str,
    tree: &tree_sitter::Tree,
    file_path: &str,
) -> Result<Vec<TestEndpoint>, QueryError> {
    let mut list_test_endpoints = Vec::new();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());
    while let Some(match_) = matches.next() {
        let mut test_endpoint = TestEndpoint {
            name: String::new(),
            file_path: file_path.to_string(),
        };
        for capture in match_.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let text = capture
                .node
                .utf8_text(source_code.as_bytes())
                .map_err(|e| QueryError::CaptureNodeError(e))?
                .to_string();
            test_endpoint.file_path = file_path.to_string();
            match capture_name {
                "test.name" => {
                    test_endpoint.name = text;
                }
                _ => {}
            }
        }
        list_test_endpoints.push(test_endpoint);
    }
    Ok(list_test_endpoints)
}
