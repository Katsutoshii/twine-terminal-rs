pub use crate::conditional::{branch_len, take_branch};
pub use crate::error::ValidationError;
pub use crate::state::update_state;
pub use crate::structs::{Branches, Config, Passage, PassageLine, Story};
pub use crate::validate::validate;
pub use colored::*;

pub struct Runner<'r> {
    pub config: &'r mut Config,
    pub story: &'r Story,
    pub line: usize,
    pub passage: &'r Passage,
    pub lines: Vec<&'r PassageLine>,
}

impl<'r> Runner<'r> {
    pub fn new(config: &'r mut Config, story: &'r Story) -> Self {
        // Flatten dialogue lines
        let passage = &story[&config.passage];
        let mut runner = Self {
            config,
            story,
            line: 0,
            lines: vec![],
            passage,
        };
        runner.load_lines(passage);
        runner
    }

    fn load_lines(&mut self, lines: &'r [PassageLine]) {
        for line in lines {
            match line {
                PassageLine::Branches(branches) => {
                    self.lines.push(&line);
                    for (_expression, branch_lines) in branches {
                        self.load_lines(branch_lines)
                    }
                }
                _ => self.lines.push(&line),
            }
        }
    }
    fn goto(&mut self, passage_name: &str) {
        self.config.passage = passage_name.to_string();
        self.config.line = 0;
        self.passage = &self.story[&self.config.passage];
        self.lines = vec![];
        self.load_lines(self.passage);
    }

    fn handle_line(&mut self, input: &str, line: &'r PassageLine) -> &'r PassageLine {
        match line {
            // When a choice is encountered, it should first be returned for display.
            // Second time its encountered,
            PassageLine::SetCmd(set) => {
                update_state(&mut self.config.state, &set.set).unwrap();
                self.config.line += 1;
                &PassageLine::Continue
            }
            PassageLine::Choices(choices) => {
                if choices.choices.contains_key(input) {
                    self.goto(&choices.choices[input]);
                    &PassageLine::Continue
                } else if input.is_empty() {
                    line
                } else {
                    &PassageLine::InvalidChoice
                }
            }
            PassageLine::Branches(branches) => {
                take_branch(&mut self.config, branches).unwrap();
                &PassageLine::Continue
            }
            PassageLine::Goto(goto) => {
                self.goto(&goto.goto);
                &PassageLine::Continue
            }
            _ => {
                // For all others, progress to the next dialog line.
                self.config.line += 1;
                line
            }
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
            #[cfg(debug_assertions)]
            {
                println!(
                    "{}",
                    format!("Input {}", curr_input).italic().bright_black()
                );
                println!("{}", format!("{:?}", self.config).italic().bright_black());
            }

            if self.config.line >= self.lines.len() {
                return None;
            }
            result = self.handle_line(curr_input, self.lines[self.config.line]);
            curr_input = "";
        }
        Some(result)
    }
}
