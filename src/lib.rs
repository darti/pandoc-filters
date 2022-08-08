use pandoc_types::definition::Pandoc;

pub fn dummy(input: String) -> String {
    input
}

pub fn echo(input: String) -> String {
    let doc = serde_json::from_str::<Pandoc>(&input).unwrap();

    serde_json::to_string(&doc).unwrap()
}

// pub fn repeat(input: String) -> String {

// }
