use crate::Operation;
use crate::config::Config;

pub struct NumberLines
{
    config: Config,
    next: Option<Box<dyn Operation>>,
}

impl NumberLines
{
    pub fn new(config: Config, next: Option<Box<dyn Operation>>) -> Self {
        NumberLines { config, next }
    }
}

impl Operation for NumberLines
{
    fn handle(&self, contents: String) -> String {
        let mut buffer: String = String::new();
        let mut counter = 1;
        for line in contents.lines() {
            let mut new_line: String = counter.to_string();
            new_line.push_str("  ");
            new_line.push_str(line);
            buffer.push_str(new_line.as_str());
            buffer.push('\n');
            counter += 1;
        }
        buffer
    }

    fn execute(&mut self, mut contents: String) -> String {
        if !self.config.n {
            ()
        } else if !self.config.b && self.config.n {
            contents = self.handle(contents);
        } else {
            ()
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
