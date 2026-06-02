use anyhow::Result;
use rayon::prelude::*;
use scanner::collect_python_files;
use std::collections::HashSet;
use std::path::Path;

pub mod arguments;
pub mod errors;
pub mod linker;
pub mod parser;
pub mod query;
pub mod scanner;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub fn run_scan<T, F>(root: &Path, test_only: bool, map_fn: F) -> Result<Vec<T>>
where
    T: Send,
    F: Fn(&str, &tree_sitter::Tree, &str) -> Vec<T> + Sync + Send,
{
    let python_files = collect_python_files(root, test_only)?;

    let results: Vec<T> = python_files
        .par_iter()
        .filter_map(|file| {
            let source_code = std::fs::read_to_string(file).ok()?;
            let file_path = file.to_string_lossy();
            let parsed = parser::parse_file(&source_code)
                .map_err(|e| eprintln!("warn: could not parse {file_path}: {e}"))
                .ok()?;
            let file_path = file.to_str()?;
            Some(map_fn(&source_code, &parsed, file_path))
        })
        .flatten()
        .collect();

    Ok(results)
}
pub fn scan_endpoints(root: &Path) -> Result<Vec<query::Endpoint>> {
    let language = tree_sitter::Language::from(tree_sitter_python::LANGUAGE);

    let decorator_query = tree_sitter::Query::new(&language, query::DECORATOR_QUERY)?;
    let test_query = tree_sitter::Query::new(&language, linker::TEST_QUERY)?;

    let mut endpoints = run_scan(root, false, |src, tree, path| {
        query::run_query(&decorator_query, src, tree, path)
            .unwrap_or_else(|e| { eprintln!("warn: {path}: {e}"); vec![] })
    })?;

    let test_names = run_scan(root, true, |src, tree, path| {
        linker::run_test_query(&test_query, src, tree, path)
            .unwrap_or_else(|e| { eprintln!("warn: {path}: {e}"); vec![] })
            .into_iter()
            .map(|t| t.name)
            .collect()
    })?;

    let test_set: HashSet<String> = test_names.into_iter().collect();
    for ep in &mut endpoints {
        let prefix = format!("test_{}", ep.name);
        ep.has_test = test_set.iter().any(|t| t.starts_with(&prefix));
    }

    Ok(endpoints)
}

// pyo3 binding - only compiled when feature is enabled
#[cfg(feature = "pyo3")]
#[pyfunction]
fn check_endpoints(root: &str) -> PyResult<String> {
    let path = Path::new(root);
    let endpoints = scan_endpoints(path)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    serde_json::to_string_pretty(&endpoints)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

#[cfg(feature = "pyo3")]
#[pymodule]
fn haki(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check_endpoints, m)?)?;
    Ok(())
}