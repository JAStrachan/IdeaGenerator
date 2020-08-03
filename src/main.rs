fn main() {
    println!("{}", separate_sentence(String::from("Sentence to split")));
}

fn separate_sentence(sentence: String) -> String {
    let split_sentence: Vec<&str> = sentence.split_whitespace().collect();
    fill_word();
    split_sentence.into_iter().collect()
}

fn fill_word() -> &'static str {
    "Hello"
}
