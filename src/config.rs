#[derive(Clone)]
pub struct Config {
    /// Number the non-blank output lines, starting at 1.
    pub b: bool,
    /// Squeeze multiple adjacent empty lines, causing the output to be single spaced.
    pub s: bool,
    /// Number the output lines, starting at 1.
    pub n: bool,
}

impl Config {
    pub fn new(b: bool, s: bool, n: bool) -> Self {
        Config { b, s, n }
    }
}