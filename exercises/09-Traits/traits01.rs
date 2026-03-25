// 1. Implement the `Summary` trait for the `BlogPost` struct
// 2. Implement the `fmt::Display` trait for `BlogPost`

use std::fmt;

trait Summary {
    fn summarise(&self) -> String;
}

pub struct BlogPost {
    pub title: String,
    pub author: String,
    pub content: String,
}

// TODO: Implement the Summary trait for BlogPost


// TODO: Implement the Display trait for BlogPost
// The Display should show the title, author, and first 50 characters of content



fn main() {
    let post = BlogPost {
        title: "Rust is great".to_string(),
        author: "Alice".to_string(),
        content: "Rust is a systems programming language that runs blazingly fast.".to_string(),
    };

    // Using the Summary trait
    assert_eq!(post.summarise(), "Rust is great by Alice");
    
    // Using the Display trait
    let display_str = format!("{}", post);
    assert!(display_str.contains("Rust is great by Alice"));
    assert!(display_str.contains("Rust is a systems programming language that runs b"));
}
