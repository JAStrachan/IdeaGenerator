use regex::Regex;

fn main() {
    println!("{}", separate_sentence(String::from("Sentence to @noun")));
}

fn separate_sentence(sentence: String) -> String {
    let mut new_sentence = sentence.clone();

    for mat in Regex::new(r"@\w*").unwrap().find_iter(&sentence) {
        let word = &sentence[mat.start()..mat.end()];
        let new_word = fill_word(word);
        new_sentence.replace_range(mat.range(), new_word)
    }

    new_sentence
}

fn fill_word(type_of_word: &str) -> &'static str {
    match type_of_word {
        "@noun" => "hospital",
        _ => "Not a templated character",
    }
}
