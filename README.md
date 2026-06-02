# haki

Haki is a fast, zero-runtime static analysis tool that scans your FastAPI project and tells you which endpoints have no test written for them, before you run a single line of code.

Built in Rust, exposed as a Python package.


## Why

`pytest --cov` tells you coverage *after* running your entire test suite. That requires a running server, dependencies, databases, all of it.

Haki answers a simpler question earlier: **did someone actually write a test function for this endpoint?** It runs in milliseconds, needs no server, and is perfect as a pre-commit hook or CI lint step on every PR. Please keep in my mind that i am a student and know for sure that this may not be the best code and could've been most likely better in the eyes of alot of very good programmers, so please make a issue! Hopefully i can learn alot and improve this project.



## Installation

```bash
pip install haki
```

---

## Usage

### As a Python package

```python
import haki
import json

results = json.loads(haki.check_endpoints("/path/to/your/fastapi/project"))

for ep in results:
    status = "✓" if ep["has_test"] else "✗"
    print(f"{status} {ep['method']} {ep['path']} — {ep['name']}")
```

### Output

```json
[
  {
    "name": "get_workflow",
    "path": "/{workflow_id}",
    "method": "Get",
    "file_path": "/project/routers/workflows.py",
    "has_test": true
  },
  {
    "name": "delete_workflow",
    "path": "/{workflow_id}",
    "method": "Delete",
    "file_path": "/project/routers/workflows.py",
    "has_test": false
  }
]
```

### In CI (fail the pipeline if any endpoint is untested)

```python
import haki
import json
import sys

results = json.loads(haki.check_endpoints("."))
missing = [ep for ep in results if not ep["has_test"]]

if missing:
    print("Missing tests for:")
    for ep in missing:
        print(f"  ✗ {ep['method']} {ep['path']} ({ep['name']})")
    sys.exit(1)
```

---

## How it works

Haki uses tree-sitter to parse your Python source files into AST trees without executing any code. It then:

1. Scans all `.py` files for FastAPI route decorators (`@app.get`, `@router.post` etc.) and extracts the function name, path, and HTTP method
2. Scans your `tests/` or `test/` directory for functions named `test_*`
3. Matches each endpoint to a test by prefix — `get_workflow` is considered tested if any function named `test_get_workflow*` exists

---

## Naming convention

Haki follows pytest convention. Your test functions must be named `test_<endpoint_function_name>`. Variants like `test_get_workflow_returns_404` or `test_get_workflow_unauthorised` are all matched correctly.

```python
# endpoint
@router.get("/{workflow_id}")
async def get_workflow(workflow_id: str):
    ...

# all of these count as a test for get_workflow
def test_get_workflow(): ...
def test_get_workflow_not_found(): ...
def test_get_workflow_returns_401(): ...
```

---

## Requirements

- Python 3.9+
- FastAPI project using decorator-based routing (`@app.get`, `@router.post` etc.)
- Tests follow `test_<function_name>` naming convention in a `tests/` or `test/` directory

---

## Limitations

- FastAPI only (Flask, Django not supported yet)
- Matches by function name convention, not by which routes are actually called in tests
- Does not replace `pytest --cov` — use both
- Plugin system will be added in the future and custom naming convention

---

## License

MIT
