use json_parser_analyzer::*;

#[test]
fn test_compare_json_structures_similar() {
    let file1_content = r#"{
        "name": "Test User",
        "age": 30,
        "isStudent": false,
        "courses": [
            {
                "title": "Rust Programming",
                "credits": 3
            },
            {
                "title": "Advanced JSON",
                "credits": 4
            }
        ]
    }"#;
    let file2_content = r#"{
        "name": "Another User",
        "age": 25,
        "isStudent": true,
        "courses": [
            {
                "title": "Web Development",
                "credits": 3
            },
            {
                "title": "Database Systems",
                "credits": 4
            }
        ]
    }"#;

    std::fs::write("test_similar_1.json", file1_content).unwrap();
    std::fs::write("test_similar_2.json", file2_content).unwrap();

    let result = compare_json_structures("test_similar_1.json", "test_similar_2.json").unwrap();
    assert_eq!(result, true);

    std::fs::remove_file("test_similar_1.json").unwrap();
    std::fs::remove_file("test_similar_2.json").unwrap();
}

#[test]
fn test_compare_json_structures_different() {
    let file1_content = r#"{
        "name": "Test User",
        "age": 30
    }"#;
    let file2_content = r#"{
        "user_id": 123,
        "username": "testuser"
    }"#;

    std::fs::write("test_different_1.json", file1_content).unwrap();
    std::fs::write("test_different_2.json", file2_content).unwrap();

    let result = compare_json_structures("test_different_1.json", "test_different_2.json").unwrap();
    assert_eq!(result, false);

    std::fs::remove_file("test_different_1.json").unwrap();
    std::fs::remove_file("test_different_2.json").unwrap();
}
