use crate::Operation;
use crate::config::Config;


pub struct Squeeze<'config>
{
    config: &'config Config,
    next: Option<Box<dyn Operation<'config> + 'config>>,
}

impl<'config> Squeeze<'config>
{
    pub fn new(config: &'config Config, next: Option<Box<dyn Operation<'config> + 'config>>) -> Self {
        Squeeze { config, next }
    }
}


impl<'config> Operation<'config> for Squeeze<'config>
{
    fn handle(&self, contents: String) -> String {
        let mut buffer: String = String::new();
        let mut prev_new_line: bool = false;
        for line in contents.lines() {
            match line {
                "" => {
                    if prev_new_line {
                        continue;
                    } else {
                        buffer.push('\n');
                        prev_new_line = true;
                    }
                }
                _ => {
                    buffer.push_str(line);
                    buffer.push('\n');
                    prev_new_line = false;
                }
            }
        }
        buffer
    }

    fn execute(&mut self, mut contents: String) -> String {
        if !self.config.s {
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
