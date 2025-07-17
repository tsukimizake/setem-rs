use std::collections::HashSet;
use std::fs;
use tree_sitter::{Parser, Query, QueryCursor};
use streaming_iterator::StreamingIterator;

pub fn extract_record_fields(tree: &tree_sitter::Tree, source: &str) -> HashSet<String> {
    let mut identifiers = HashSet::new();
    
    let query_str = r#"
        (field_type
          (lower_case_identifier) @field_name)
        
        (field
          (lower_case_identifier) @field_name)
    "#;
    
    let language = tree_sitter_elm::LANGUAGE.into();
    let query = Query::new(&language, query_str).expect("Error creating query");
    let mut cursor = QueryCursor::new();
    
    let mut captures = cursor.captures(&query, tree.root_node(), source.as_bytes());
    
    while let Some((match_, capture_index)) = captures.next() {
        let capture = &match_.captures[*capture_index];
        let text = capture.node.utf8_text(source.as_bytes()).unwrap_or("");
        if !text.is_empty() {
            identifiers.insert(text.to_string());
        }
    }
    
    identifiers
}

pub fn process_elm_file(file_path: &str, parser: &mut Parser, identifiers: &mut HashSet<String>) {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            if let Some(tree) = parser.parse(&content, None) {
                let file_identifiers = extract_record_fields(&tree, &content);
                identifiers.extend(file_identifiers);
            } else {
                eprintln!("Warning: Failed to parse {}", file_path);
            }
        }
        Err(e) => {
            eprintln!("Warning: Failed to read {}: {}", file_path, e);
        }
    }
}

pub fn process_elm_directory(dir_path: &str, parser: &mut Parser, identifiers: &mut HashSet<String>) {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "elm" {
                            if let Some(path_str) = path.to_str() {
                                process_elm_file(path_str, parser, identifiers);
                            }
                        }
                    }
                } else if path.is_dir() {
                    if let Some(path_str) = path.to_str() {
                        process_elm_directory(path_str, parser, identifiers);
                    }
                }
            }
        }
    }
}

pub fn generate_setters(identifiers: &HashSet<String>, prefix: &str) -> String {
    let mut setters = String::new();
    
    for identifier in identifiers {
        let setter_def = format!(
            "{}{} : a -> {{ b | {} : a }} -> {{ b | {} : a }}\n{}{} value__ record__ =\n    {{ record__ | {} = value__ }}\n\n",
            prefix, identifier, identifier, identifier, prefix, identifier, identifier
        );
        setters.push_str(&setter_def);
    }
    
    setters
}

pub fn setup_parser() -> Parser {
    let mut parser = Parser::new();
    let language = tree_sitter_elm::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Elm grammar");
    parser
}