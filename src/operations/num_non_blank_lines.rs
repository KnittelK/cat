use crate::Operation;
use crate::config::Config;

pub struct NumberNonBlankLines<'config> {
    config: &'config Config,
    next: Option<Box<dyn Operation<'config> + 'config>>,
}

impl<'config> NumberNonBlankLines<'config>
{
    pub fn new(config: &'config Config, next: Option<Box<dyn Operation<'config> + 'config>>) -> Self {
        NumberNonBlankLines { config, next }
    }
}


impl<'config> Operation<'config> for NumberNonBlankLines<'config>
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

    fn next(&mut self) -> &mut Option<Box<dyn Operation<'config> + 'config>> {
        &mut self.next
    }
}
