use crate::Operation;
use crate::config::Config;

pub struct NumberNonBlankLines {
    config: Config,
    next: Option<Box<dyn Operation>>,
}

impl NumberNonBlankLines
{
    pub fn new(config: Config, next: Option<Box<dyn Operation>>) -> Self {
        NumberNonBlankLines { config, next }
    }
}


impl Operation for NumberNonBlankLines
{
    fn handle(&self, contents: String) -> String {
        let mut buffer: String = String::new();
        let mut counter = 1;
        for line in contents.lines() {
            match line {
                "" => {
                    buffer.push_str(line);
                    buffer.push('\n');
                }
                _ => {
                    let mut new_line: String = counter.to_string();
                    new_line.push_str("  ");
                    new_line.push_str(line);
                    buffer.push_str(new_line.as_str());
                    buffer.push('\n');
                    counter += 1
                }
            }
        }
        buffer
    }

    fn execute(&mut self, mut contents: String) -> String {
        if !self.config.b {
            ();
        } else {
            contents = self.handle(contents)
        }
        if let Some(next) = &mut self.next() {
            next.execute(contents)
        } else {
            contents
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Operation>> {
        &mut self.next
    }
}
