use tree_sitter::{Parser, Tree};

fn main() {
    let mut parser = Parser::new();
    let language = tree_sitter_elm::LANGUAGE;
    parser.set_language(&language.into()).expect("Error loading Elm grammar");

    let elm_code = r#"
module Main exposing (main)

import Html exposing (Html, text)

main : Html msg
main =
    text "Hello, Elm!"

greet : String -> String
greet name =
    "Hello, " ++ name ++ "!"

numbers : List Int
numbers =
    [ 1, 2, 3, 4, 5 ]

add : Int -> Int -> Int
add x y =
    x + y
"#;

    let tree = parser.parse(elm_code, None).expect("Error parsing Elm code");
    let root_node = tree.root_node();
    
    println!("Root node: {:?}", root_node.kind());
    println!("Root node range: {:?}", root_node.range());
    println!("Number of children: {}", root_node.child_count());
    
    print_tree(&tree, elm_code, root_node, 0);
}

fn print_tree(tree: &Tree, source: &str, node: tree_sitter::Node, depth: usize) {
    let indent = "  ".repeat(depth);
    let node_text = node.utf8_text(source.as_bytes()).unwrap_or("<error>");
    
    if node.child_count() == 0 {
        println!("{}{}: \"{}\"", indent, node.kind(), node_text.trim());
    } else {
        println!("{}{}", indent, node.kind());
    }
    
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            print_tree(tree, source, child, depth + 1);
        }
    }
}
