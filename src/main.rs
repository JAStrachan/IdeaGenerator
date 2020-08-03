fn main() {
    println!("{}", separate_sentence(String::from("Sentence to split")));
}

fn separate_sentence(sentence: String) -> String {
    let split_sentence: Vec<&str> = sentence.split_whitespace().collect();
    fill_word();
    let new_sentence: String = split_sentence.into_iter().collect();
    new_sentence
}

fn fill_word() -> &'static str {
    "Hello"
}
