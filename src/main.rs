use rand::Rng;
use regex::Regex;
use std::error::Error;
use std::fs;

fn main() {
    println!(
        "{}",
        separate_sentence(String::from(
            "Help I need to run to that , @verbs, , @buildings, help goodness me I need another @verbs to test out"
        ))
    );
}

//TODO need to add better error handling so it can be handled in the part that spits back out to the UI
//Write better names for the functions
//seoarate_sentences will be the main entry point to the library so probably name it idea_generator
fn separate_sentence(sentence: String) -> String {
    let ast = build_ast(sentence);

    let new_sentence_ast: Vec<String> = ast.into_iter().map(|word| fill_word(&word)).collect();
    new_sentence_ast.into_iter().map(|i| i).collect::<String>()
}

fn build_ast(sentence: String) -> Vec<String> {
    let mut ast: Vec<String> = Vec::new();
    let mut last_index = 0;

    for mat in Regex::new(r"@\w*").unwrap().find_iter(&sentence) {
        ast.push(String::from(&sentence[last_index..mat.start()]));
        ast.push(String::from(&sentence[mat.start()..mat.end()]));
        last_index = mat.end();
    }
    ast.push(String::from(&sentence[last_index..sentence.len()]));
    ast
}

fn fill_word(type_of_word: &str) -> String {
    if &type_of_word[0..1] == "@" {
        let file_name = String::from(&type_of_word[1..type_of_word.len()]) + ".txt";
        println!("Opening {}", file_name);
        random_word(&file_name)
    } else {
        String::from(type_of_word)
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
