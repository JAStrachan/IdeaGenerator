use rand::Rng;
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
        "@noun" => random_noun(),
        _ => "Not a templated character",
    }
}

fn random_noun() -> &'static str {
    let mut rng = rand::thread_rng();
    let list_of_nouns = vec!["thing", "hospital", "monkey", "banana"];
    let random_number = rng.gen_range(0, list_of_nouns.len());
    return list_of_nouns[random_number];
}
