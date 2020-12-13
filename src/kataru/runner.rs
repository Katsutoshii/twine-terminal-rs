pub use crate::conditional::{branch_len, take_branch};
pub use crate::error::ValidationError;
pub use crate::state::update_state;
pub use crate::structs::{Branches, Config, Passage, PassageLine, Story};
pub use crate::validate::validate;
pub use colored::*;

pub struct Runner {
    pub config: Config,
    pub story: Story,
    pub line: usize,
}

impl Runner {
    pub fn load(config_str: &str, story_str: &str) -> Result<Self, serde_yaml::Error> {
        // Flatten dialogue lines
        Ok(Self {
            config: serde_yaml::from_str(&config_str)?,
            story: serde_yaml::from_str(&story_str)?,
            line: 0,
        })
    }

    /// Given a relative line number, gets a reference to the passage line
    /// at that line number.
    /// Returns None if the line is out of bounds.
    fn get_branch_line(line: usize, branches: &Branches<PassageLine>) -> Option<&PassageLine> {
        let mut curr_line = line;
        for (_expression, branch_lines) in branches {
            let branch_line = Self::get_line(curr_line, branch_lines);
            if branch_line.is_some() {
                return branch_line;
            }
            curr_line -= branch_len(branch_lines);
        }
        None
    }

    /// Given a relative line number, returns a reference to that line.
    /// If
    /// Returns None if the line is out of bounds.
    fn get_line(line: usize, lines: &[PassageLine]) -> Option<&PassageLine> {
        // If we can find the line at this level, return that line.
        if line < lines.len() {
            return Some(&lines[line]);
        }
        let last_line = lines.last();
        if let Some(PassageLine::Branches(branches)) = last_line {
            Self::get_branch_line(line - lines.len(), branches)
        } else {
            None
        }
    }
    // Processes input from the previous line, and returns the next line.
    // Say the line 0 is a choice.
    // First call of next returns the choice, and line should stay at 0.
    // Don't progress until a valid choice is made.
    // Then we call next("decision")
    //
    // Say the first line is a branch.
    // Evaluate the branch, modify the line and jump to the appropriate line number.
    // Then return next.
    pub fn next(&mut self, input: &str) -> Option<&PassageLine> {
        let mut result = &PassageLine::Continue;
        let mut curr_input = input;
        while result == &PassageLine::Continue {
            println!("{}", format!("{:?}", self.config).italic().bright_black());
            let passage = &self.story[&self.config.passage];

            // If line is None, this is the end of the story.
            let line_or = Self::get_line(self.config.line, &passage);
            if line_or.is_none() {
                return None;
            }

            // Otherwise process the line.
            let line = line_or.unwrap();
            result = match line {
                // When a choice is encountered, it should first be returned for display.
                // Second time its encountered,
                PassageLine::SetCmd(set) => {
                    update_state(&mut self.config.state, &set.set).unwrap();
                    self.config.line += 1;
                    &PassageLine::Continue
                }
                PassageLine::Choices(choices) => {
                    if choices.choices.contains_key(curr_input) {
                        self.config.passage = choices.choices[curr_input].to_string();
                        self.config.line = 0;
                        &PassageLine::Continue
                    } else {
                        println!(
                            "{} was not one of the chocies {:?}",
                            curr_input, choices.choices
                        );
                        line
                    }
                }
                PassageLine::Branches(branches) => {
                    take_branch(&mut self.config, branches).unwrap();
                    &PassageLine::Continue
                }
                PassageLine::Goto(goto) => {
                    self.config.passage = goto.goto.to_string();
                    self.config.line = 0;
                    &PassageLine::Continue
                }
                _ => {
                    // For all others, progress to the next dialog line.
                    self.config.line += 1;
                    line
                }
            };
            curr_input = "";
        }
        Some(result)
    }
}

#[test]
fn test_load() {
    let story_str = include_str!("../../story/story.yml");
    let config_str = include_str!("../../story/config.yml");
    let runner = Runner::load(config_str, story_str).unwrap();
    match validate(&runner.config, &runner.story) {
        Err(e) => assert!(false, format!("{}", e)),
        Ok(_) => assert!(true),
    }
}
