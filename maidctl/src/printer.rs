/*#[macro_export]
macro_rules! show_error_message {
    ($txt:literal) => {{
        eprintln!("{}", owo_colors::OwoColorize::if_supports_color(
            &format!("Error: {}", $txt),
            owo_colors::Stream::Stderr,
            owo_colors::OwoColorize::bright_red
        ));
    }};
    ($($arg:tt)*) => {{
        eprintln!("{}", owo_colors::OwoColorize::if_supports_color(
            &format!("Error: {}", format!($($arg)*)),
            owo_colors::Stream::Stderr,
            owo_colors::OwoColorize::bright_red
        ));
    }};
}

#[macro_export]
macro_rules!
}*/

use maidono_core::utils::{Error, ErrorPrinter};
use owo_colors::{OwoColorize, Stream};
use std::fmt::Display;
use std::fmt::Write;

pub struct Printer {
    indent: usize,
    new_line: bool,
}

impl Printer {
    pub fn new() -> Self {
        Self {
            indent: 0,
            new_line: true,
        }
    }

    fn pad(&self) -> String {
        if self.new_line {
            "  ".repeat(self.indent)
        } else {
            String::new()
        }
    }

    pub fn println(&mut self) -> &mut Self {
        println!();
        self.new_line = true;
        self
    }

    pub fn print_text<D: Display>(&mut self, value: D) -> &mut Self {
        print!(" {}{}", self.pad(), value);
        self.new_line = false;
        self
    }

    pub fn println_text<D: Display>(&mut self, value: D) -> &mut Self {
        println!(" {}{}", self.pad(), value);
        self.new_line = true;
        self
    }

    pub fn print_em_text<D: Display>(&mut self, value: D) -> &mut Self {
        print!(" {}{}", self.pad(), value.bold());
        self.new_line = false;
        self
    }

    pub fn println_list<D: IntoIterator>(&mut self, value: D) -> &mut Self
    where
        <D as IntoIterator>::Item: std::fmt::Display,
    {
        print!(" {}", self.pad());
        let mut first = true;
        for elem in value {
            if !first {
                print!(", ")
            }
            print!("{}", elem);
            first = false;
        }
        println!();
        self.new_line = true;
        self
    }

    pub fn print_multiline<D: Display>(&mut self, value: D) -> &mut Self {
        let mut multiline_string = String::new();
        write!(multiline_string, "{}", value).unwrap();

        for line in multiline_string.lines() {
            println!(" {}{}", self.pad(), line);
        }
        self.new_line = true;
        self
    }

    pub fn print_very_compact_error(&mut self, error: Error) -> &mut Self {
        error.display_very_compact(self);
        self
    }

    pub fn print_detailed_error(&mut self, error: Error) -> &mut Self {
        error.display_detailed(self);
        self
    }
}

pub struct SensitiveStr<'a> {
    value: &'a str,
}

impl<'a> From<&'a str> for SensitiveStr<'a> {
    fn from(value: &'a str) -> Self {
        Self { value }
    }
}

impl Display for SensitiveStr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from("*").repeat(self.value.len()))
    }
}

impl ErrorPrinter for Printer {
    fn indent(&mut self) -> &mut Self {
        self.indent += 1;
        self
    }

    fn unindent(&mut self) -> &mut Self {
        self.indent -= 1;
        self
    }

    fn print_error_inline<D: Display>(&mut self, text: D) -> &mut Self {
        eprint!(
            " {}{}",
            self.pad(),
            text.if_supports_color(Stream::Stderr, |x| x.bright_red())
        );
        self.new_line = false;
        self
    }

    fn print_error<D: Display>(&mut self, text: D) -> &mut Self {
        eprintln!(
            " {}{}",
            self.pad(),
            text.if_supports_color(Stream::Stderr, |x| x.bright_red())
        );
        self.new_line = true;
        self
    }
}
