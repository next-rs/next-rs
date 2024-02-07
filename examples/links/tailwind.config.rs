(
    content: [
        "src/**/*.rs",
        "index.html"
    ],
    extend_collection_options: Some({
        "rs": Regex(r#"(?:class)=(?:["]\W+\s*(?:\w+)\()?["]([^"]+)["]"#)
    })
)