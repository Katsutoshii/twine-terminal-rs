pub mod kataru;
use colored::*;
use kataru::*;
use std::io::{stdin, stdout, Write};

fn get_input(input: &mut String) {
    let _ = stdout().flush();
    *input = String::new();
    stdin()
        .read_line(input)
        .expect("Did not enter a correct string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
}

fn await_key() {
    let mut input = String::new();
    println!("{}", "Press any key to continue...".italic().bright_black());
    stdin().read_line(&mut input).expect("Invalid input.");
}

fn main() {
    // Load the story.
    println!("{}", "Loading story...".bold().cyan());
    let story_str = include_str!("../story/story.yml");
    let config_str = include_str!("../story/config.yml");
    let mut runner = Runner::load(config_str, story_str).unwrap();

    // Validate the story.
    println!("{}", "Validating story...".bold().cyan());
    let msg = match validate(&runner.config, &runner.story) {
        Err(e) => format!("{}", e).red(),
        Ok(_) => "Validated story successfully.".bold().green(),
    };
    println!("{}\n", msg);

    let mut input = String::new();
    loop {
        match runner.next(&input) {
            Some(line) => match &line {
                PassageLine::Text(text) => {
                    println!("{}", text.italic());
                    await_key();
                }
                PassageLine::Dialogue(dialogue) => {
                    let (name, quote) = dialogue.iter().next().unwrap();
                    println!("{}: {}", name.bold().yellow(), quote);
                    await_key();
                }
                PassageLine::Choices(choices) => {
                    for (choice, _passage_name) in &choices.choices {
                        println!("{}", choice.cyan());
                    }
                    print!("{}", "Enter your choice: ".magenta());
                    get_input(&mut input);
                }
                _ => (),
            },
            None => break,
        }
    }
}
