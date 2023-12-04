mod config;
mod operations;

use std::fs;
use std::{io, process};
use std::io::Write;
use clap::Parser;
use crate::config::Config;
use crate::operations::{
    num_lines::NumberLines,
    num_non_blank_lines::NumberNonBlankLines,
    squeeze::Squeeze,
    operation_trait::Operation,
};

#[derive(Parser)]
struct Cat {
    files: Vec<String>,

    /// Number the non-blank output lines, starting at 1.
    #[arg(short)]
    b: bool,

    /// Squeeze multiple adjacent empty lines, causing the output to be single spaced.
    #[arg(short)]
    s: bool,

    /// Number the output lines, starting at 1.
    #[arg(short)]
    n: bool,
}

impl Cat {
    pub fn run(&self) {
        let config = Config::new(self.b, self.s, self.n);
        let number_lines: Option<Box<dyn Operation>> = Some(Box::new(NumberLines::new(config.clone(), None)));
        let number_non_blank_lines: Option<Box<dyn Operation>> = Some(Box::new(NumberNonBlankLines::new(config.clone(), number_lines)));
        let mut squeeze = Squeeze::new(config.clone(), number_non_blank_lines);

        let mut output: String = String::new();
        for file_name in self.files.iter() {
            let mut contents = fs::read_to_string(file_name).unwrap_or_else(|err| {
                eprint!("An error occurred: {err}");
                io::stdout().flush().unwrap();
                process::exit(1);
            }
            );
            contents = squeeze.execute(contents);

            output.push_str(&contents);
        }
        output.pop();
        println!("{}", output);
    }
}


fn main() {
    Cat::parse().run();
}
