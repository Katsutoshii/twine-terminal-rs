pub mod kataru;
use colored::*;
use kataru::*;
use std::io::{stdin, stdout, Write};

fn get_input(input: &mut String) {
    let _ = stdout().flush();
    *input = String::new();
    stdin().read_line(input).expect("Invalid input");
    loop {
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        } else if let Some('\r') = input.chars().next_back() {
            input.pop();
        } else {
            break;
        }
    }
}

fn await_key(input: &mut String) {
    get_input(input);
    *input = String::new();
}

fn main() {
    // Load the story.
    println!("{}", "Loading story...".bold().cyan());
    let story_str = include_str!("../story/story.yml");
    let config_str = include_str!("../story/config.yml");
    let story: Story = serde_yaml::from_str(&story_str).unwrap();
    let mut config: Config = serde_yaml::from_str(&config_str).unwrap();
    let mut runner = Runner::new(&mut config, &story);

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
                    await_key(&mut input);
                }
                PassageLine::Dialogue(dialogue) => {
                    let (name, quote) = dialogue.iter().next().unwrap();
                    println!("{}: {}", name.bold().yellow(), quote);
                    await_key(&mut input);
                }
                PassageLine::Choices(choices) => {
                    for (choice, _passage_name) in &choices.choices {
                        println!("{}", choice.cyan());
                    }
                    print!("{}", "Enter your choice: ".magenta());
                    get_input(&mut input);
                }
                PassageLine::InvalidChoice => {
                    print!(
                        "{}",
                        format!("Invalid choice '{}', try again: ", input).magenta()
                    );
                    get_input(&mut input);
                }
                _ => (),
            },
            None => break,
        }
    }
}
