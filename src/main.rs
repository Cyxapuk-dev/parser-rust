use json_parser_analyzer::{
    compare_json_structures, generate_large_json_file, stream_parse_and_filter_json_file,
};
use std::error::Error as StdError;

fn main() -> Result<(), Box<dyn StdError>> {
    let large_test_file_path = "large_test.json";
    let num_items = 100_000;

    println!("Generating large JSON file with {} items...", num_items);
    generate_large_json_file(large_test_file_path, num_items)?;
    println!("Large JSON file generated: {}\n", large_test_file_path);

    let filter_key = "name";
    let filter_value = "item100";

    match stream_parse_and_filter_json_file(large_test_file_path, filter_key, filter_value) {
        Ok(_) => println!("Successfully streamed, parsed, and filtered large JSON.\n"),
        Err(e) => eprintln!("Error during streaming parsing and filtering: {}\n", e),
    }

    let file1 = "test.json";
    let file2 = "test_similar.json";
    let file3 = "test_different.json";

    match compare_json_structures(file1, file2) {
        Ok(are_similar) => {
            println!(
                "Structure comparison of {} and {}: {}",
                file1, file2, are_similar
            );
        }
        Err(e) => eprintln!("Error comparing structures: {}", e),
    }

    match compare_json_structures(file1, file3) {
        Ok(are_similar) => {
            println!(
                "Structure comparison of {} and {}: {}",
                file1, file3, are_similar
            );
        }
        Err(e) => eprintln!("Error comparing structures: {}", e),
    }

    Ok(())
}
