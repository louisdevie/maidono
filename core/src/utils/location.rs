use std::fmt::Display;

pub struct Location {
    file: String,
    line: usize,
    column: usize,
}

impl Location {
    pub fn new(file: String, line: usize, column: usize) -> Self {
        Self { file, line, column }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "in file {}, line {}, column {}",
            self.file, self.line, self.column
        ))
    }
}
