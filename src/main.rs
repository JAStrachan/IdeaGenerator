use rand::Rng;
use regex::Regex;
use std::error::Error;
use std::fs;

fn main() {
    println!(
        "{}",
        separate_sentence(String::from(
            "Help I need to run to that , @verb, , @building, help goodness me I need another @verb to test out"
        ))
    );
}

// Does work for two  + possi more @symbols but doesn't get the last part of the sentence after the last symbol
fn separate_sentence(sentence: String) -> String {
    let mut ast: Vec<&str> = Vec::new();
    let mut last_index = 0;

    for mat in Regex::new(r"@\w*").unwrap().find_iter(&sentence) {
        ast.push(&sentence[last_index..mat.start()]);
        ast.push(&sentence[mat.start()..mat.end()]);
        last_index = mat.end();
    }
    ast.push(&sentence[last_index..sentence.len()]);

    let new_sentence_ast: Vec<String> = ast.into_iter().map(|word| fill_word(word)).collect();
    new_sentence_ast.into_iter().map(|i| i).collect::<String>()
}

fn fill_word(type_of_word: &str) -> String {
    match type_of_word {
        "@building" => random_word("buildings.txt"),
        "@verb" => random_word("verbs.txt"),
        _ => String::from(type_of_word),
    }
}

fn random_word(filename: &str) -> String {
    let contents = read_file(filename).expect("Problem with reading the file");

    String::from(choose_random_word(contents.lines().collect()))
}

fn choose_random_word(list_of_words: Vec<&str>) -> &str {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0, list_of_words.len());
    return list_of_words[random_number];
}

fn read_file(filename: &str) -> Result<(String), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}
