use rand::Rng;
use regex::Regex;
use std::error::Error;
use std::fs;

fn main() {
    println!(
        "{}",
        separate_sentence(String::from(
            "Help I need to run to that , @verb, , @building, help"
        ))
    );
}

// Doesn't work on multiple tokens because it doesn't relevaluate where the next token is. Do I have to build a AST?????
fn separate_sentence(sentence: String) -> String {
    let mut new_sentence = sentence.clone();

    for mat in Regex::new(r"@\w*").unwrap().find_iter(&sentence) {
        let word = &sentence[mat.start()..mat.end()];
        let new_word = fill_word(word);

        new_sentence.replace_range(mat.range(), &new_word);
    }

    new_sentence
}

fn fill_word(type_of_word: &str) -> String {
    match type_of_word {
        "@building" => random_word("buildings.txt"),
        "@verb" => random_word("verbs.txt"),
        _ => String::from("Not a templated character"),
    }
}

fn random_word(filename: &str) -> String {
    let contents = read_file(filename).expect("Problem with reading the file");

    let mut words = Vec::new();
    for line in contents.lines() {
        words.push(line);
    }

    choose_random_word(words)
}

fn choose_random_word(list_of_words: Vec<&str>) -> String {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0, list_of_words.len());
    return String::from(list_of_words[random_number]);
}

fn read_file(filename: &str) -> Result<(String), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}
