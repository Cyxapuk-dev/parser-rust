# JSON Parser and Analyzer

This project implements a JSON parser and analyzer in Rust, supporting streaming parsing for large files, data filtering, and structural comparison of JSON files.

## Features

- **Streaming Parsing**: Efficiently parse large JSON files without loading the entire content into memory.
- **Data Filtering**: Filter JSON data based on specified keys and values.
- **Structure Comparison**: Compare the structural similarity of two JSON files.
- **Test Data Generation**: Generate large JSON files for testing purposes directly in Rust.

## Getting Started

### Prerequisites

- Rust programming language (version 1.87.0 or later)
- Cargo (Rust's package manager, usually installed with Rust)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/json_parser_analyzer.git
   cd json_parser_analyzer
   ```

2. Build the project:

   ```bash
   cargo build
   ```

## Usage

### Generating Large Test JSON Files

To generate a large JSON file for testing, the `generate_large_json_file` function is available. This function creates a file with a specified number of JSON objects, each containing an `id` and `name` field.

Example (in `src/main.rs`):

```rust
use json_parser_analyzer::generate_large_json_file;

fn main() {
    let large_test_file_path = "large_test.json";
    let num_items = 100_000;

    match generate_large_json_file(large_test_file_path, num_items) {
        Ok(_) => println!("Large JSON file generated: {}\n", large_test_file_path),
        Err(e) => eprintln!("Error generating large JSON file: {}\n", e),
    }
}
```

### Streaming Parsing and Filtering

To stream parse a large JSON file and filter items based on a key-value pair, you can use the `stream_parse_and_filter_json_file` function. The current implementation is set up to filter `Item` structs with `id` and `name` fields.

Example (in `src/main.rs`):

```rust
use json_parser_analyzer::stream_parse_and_filter_json_file;

fn main() {
    // ... (code for generating large JSON file)

    let large_test_file_path = "large_test.json";
    let filter_key = "name";
    let filter_value = "item100";

    match stream_parse_and_filter_json_file(large_test_file_path, filter_key, filter_value) {
        Ok(_) => println!("Successfully streamed, parsed, and filtered large JSON.\n"),
        Err(e) => eprintln!("Error during streaming parsing and filtering: {}\n", e),
    }
}
```

### JSON Structure Comparison

To compare the structure of two JSON files, use the `compare_json_structures` function. This function checks if two JSON files have similar structures (same keys in objects, same types for primitive values, and similar structure for array elements).

Example (in `src/main.rs`):

```rust
use json_parser_analyzer::compare_json_structures;

fn main() {
    // ... (previous code for streaming parsing)

    let file1 = "test.json";
    let file2 = "test_similar.json";
    let file3 = "test_different.json";

    match compare_json_structures(file1, file2) {
        Ok(are_similar) => {
            println!("Structure comparison of {} and {}: {}", file1, file2, are_similar);
        },
        Err(e) => eprintln!("Error comparing structures: {}", e),
    }

    match compare_json_structures(file1, file3) {
        Ok(are_similar) => {
            println!("Structure comparison of {} and {}: {}", file1, file3, are_similar);
        },
        Err(e) => eprintln!("Error comparing structures: {}", e),
    }
}
```

Ensure `test.json`, `test_similar.json`, and `test_different.json` files are present in the project root for this example to work. These files are created during the development process.

## Running Tests

To run the unit tests for the project, use the following command:

```bash
cargo test
```

## Project Structure

```
json_parser_analyzer/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── lib.rs
├── tests/
│   └── structure_comparison.rs
├── test.json
├── test_similar.json
└── test_different.json
```

## License

This project is licensed under the MIT License - see the LICENSE file for details. (Note: LICENSE file is not created in this example, but would be in a real project.)


#   - 
 
 
