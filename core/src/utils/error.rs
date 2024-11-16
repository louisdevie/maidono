use super::Location;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

pub enum Error {
    StaticMessage(&'static str),
    DynamicMessage(String),
    Because(Box<Error>, Box<Error>),
    At(Box<Error>, Location),
    Multiple(Vec<Error>),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn because<E: Into<Error>>(self, error: E) -> Self {
        Self::Because(Box::new(self), Box::new(error.into()))
    }

    pub fn at(self, location: Location) -> Self {
        Self::At(Box::new(self), location.into())
    }

    pub fn in_file(self, file: String, line: usize, column: usize) -> Self {
        Self::At(Box::new(self), Location::new(file, line, column))
    }

    pub fn and(self, other: Error) -> Self {
        match self {
            Error::Multiple(mut list) => {
                list.push(other);
                Self::Multiple(list)
            }
            any => Self::Multiple(vec![any, other]),
        }
    }

    pub fn display_very_compact<P: ErrorPrinter>(self, printer: &'_ mut P) {
        match self {
            Error::StaticMessage(msg) => printer.print_error_inline(msg),
            Error::DynamicMessage(msg) => printer.print_error_inline(msg),
            Error::Because(_, _) => printer.print_error_inline("1 error"),
            Error::At(_, _) => printer.print_error_inline("1 error"),
            Error::Multiple(errs) => printer.print_error_inline(format!("{} errors", errs.len())),
        };
    }

    pub fn display_detailed<P: ErrorPrinter>(self, printer: &'_ mut P) {
        match self {
            Error::StaticMessage(msg) => {
                printer.print_error(msg);
            }
            Error::DynamicMessage(msg) => {
                printer.print_error(msg);
            }
            Error::Because(err, cause) => {
                err.display_detailed(printer);
                printer
                    .print_error("due to the following error(s) :")
                    .indent();
                cause.display_detailed(printer);
                printer.unindent();
            }
            Error::At(err, location) => {
                err.display_detailed(printer);
                printer.indent().print_error(location).unindent();
            }
            Error::Multiple(errs) => {
                for err in errs {
                    err.display_detailed(printer);
                }
            }
        };
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::DynamicMessage(value.to_string())
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(value: serde_yaml::Error) -> Self {
        Self::DynamicMessage(value.to_string())
    }
}

impl Debug for Error {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        todo!()
    }
}

pub trait ErrorPrinter {
    fn indent(&mut self) -> &mut Self;

    fn unindent(&mut self) -> &mut Self;

    fn print_error<D: Display>(&mut self, text: D) -> &mut Self;

    fn print_error_inline<D: Display>(&mut self, text: D) -> &mut Self;
}
