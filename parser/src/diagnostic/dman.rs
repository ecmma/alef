use crate::diagnostic::err::{Diagnostic, LabeledSpan, Severity};
use owo_colors::OwoColorize;
use std::{fmt, sync::Mutex, sync::LazyLock};

// @XXX This may be changed in favour of a better alternative?
/// The warning level used to configure the global state of the singleton DiagnosticsManager.
pub static WARNING_LEVEL: LazyLock<Mutex<u8>> = LazyLock::new(|| Mutex::new(0));

pub static DIAGMAN: LazyLock<Mutex<Box<dyn DiagnosticsManager + Sync + Send>>> =
    LazyLock::new(|| Mutex::new(Box::new(DefaultDiagnosticsManager {})));

pub trait DiagnosticsManager: Sync {
    /// Publish a new diagnostic message.
    fn publish(&mut self, diag: Box<dyn Diagnostic>);
}

pub struct DefaultDiagnosticsManager {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ThemeCharacters {
    pub hbar: char,
    pub vbar: char,
    pub xbar: char,
    pub vbar_break: char,

    pub uarrow: char,
    pub rarrow: char,

    pub ltop: char,
    pub mtop: char,
    pub rtop: char,
    pub lbot: char,
    pub rbot: char,
    pub mbot: char,

    pub lbox: char,
    pub rbox: char,

    pub lcross: char,
    pub rcross: char,

    pub underbar: char,
    pub underline: char,

    pub fatal: String,
    pub error: String,
    pub warning: String,
    pub info: String,
}

impl ThemeCharacters {
    /// Fancy unicode-based graphical elements.
    pub fn unicode() -> Self {
        Self {
            hbar: '─',
            vbar: '│',
            xbar: '┼',
            vbar_break: '·',
            uarrow: '▲',
            rarrow: '▶',
            ltop: '╭',
            mtop: '┬',
            rtop: '╮',
            lbot: '╰',
            mbot: '┴',
            rbot: '╯',
            lbox: '[',
            rbox: ']',
            lcross: '├',
            rcross: '┤',
            underbar: '┬',
            underline: '─',
            fatal: "☠".into(),
            error: "×".into(),
            warning: "⚠".into(),
            info: "⚐".into(),
        }
    }
}

//
// The structure of an error message (for what concerns alef's frontend..):
// as example, a type checking error for a binary operator with code
// af::typecheck::err::BinaryOpTypeError at loc <bad_file.l:10:7>
// The anatomy of an error message:
//
//   A string representing the severity: one of Fatal, Error, Warning, Info.
//     |
//     |    |- The error code.                    |- The abstract reason
//     v    v                                     v
//   Error af::typecheck::err::BinaryOpTypeError: incompatible types     | HEADER
//
//      |- CONTEXT
//      v
//      ╭─ bad_file.l:10:7  <- Position in the file.
//   10 │ x = 4 + "this_is_a_str"; <- The culprit line in the source.
//      ·     ┬ ┬ ──────┬───────                               |
//      ·     │ │       ╰─ string                              | Optional labels with messages adding
//      ·     │ ╰─ cannot use operator '+' on these operands.  | contextual information to the error.
//      ·     ╰── int                                          |
//      ╰────
//
//      |- FOOTER
//      v
//
//      Cause: operands must coerce to the same type. <- Optional cause message.
//
//      Help: Using '+' operator on a string and a non-string does not append the integer
//            value to the string.  <- Optional help message.

impl DiagnosticsManager for DefaultDiagnosticsManager {
    fn publish<'a>(&mut self, diag: Box<dyn Diagnostic + 'a>) {
        let mut out = String::new();
        let _ = self.render(&mut out, diag.as_ref());
        eprint!("{}", out);
    }
}

impl DefaultDiagnosticsManager {
    fn render(&self, f: &mut dyn fmt::Write, diag: &(dyn Diagnostic)) -> fmt::Result {
        writeln!(f)?;
        self.header(f, diag)?;
        self.context(f, diag)?;
        self.footer(f, diag)?;

        Ok(())
    }

    fn header(&self, f: &mut dyn fmt::Write, diag: &(dyn Diagnostic)) -> fmt::Result {
        let severity = match diag.severity() {
            Some(Severity::Fatal) => "fatal".red().bold().to_string(),
            Some(Severity::Error) | None => "error".red().bold().to_string(),
            Some(Severity::Warning) => "info".yellow().bold().to_string(),
            Some(Severity::Info) => "info".bold().to_string(),
        };

        let code = match diag.code() {
            Some(c) => format!("{}", c),
            None => "".to_string(),
        };

        writeln!(
            f,
            "{} {}: {}",
            severity,
            code.bold(),
            diag.to_string().underline()
        )?;

        let charset = ThemeCharacters::unicode();

        let cause_symbol = match diag.severity() {
            Some(Severity::Fatal) => charset.fatal.red().bold().to_string(),
            Some(Severity::Error) | None => charset.error.red().bold().to_string(),
            Some(Severity::Warning) => charset.warning.yellow().bold().to_string(),
            Some(Severity::Info) => charset.info.bold().to_string(),
        };

        let cause = match diag.reason() {
            Some(c) => format!("{}", c),
            None => "".to_string(),
        };

        let opts = textwrap::Options::new(80)
            .initial_indent("")
            .subsequent_indent("  ");

        let cause = format!("{} {}", cause_symbol, cause);
        writeln!(f)?;
        writeln!(f, "{}", textwrap::fill(&cause, opts))?;
        writeln!(f)?;
        Ok(())
    }

    fn write_no_linum(
        &self,
        f: &mut dyn fmt::Write,
        width: usize,
        theme: &ThemeCharacters,
    ) -> fmt::Result {
        write!(f, " {:width$} {} ", "", theme.vbar_break, width = width)?;
        Ok(())
    }
    fn write_linum(
        &self,
        f: &mut dyn fmt::Write,
        width: usize,
        linum: usize,
        theme: &ThemeCharacters,
    ) -> fmt::Result {
        write!(
            f,
            " {:width$} {} ",
            linum.to_string().bold(),
            theme.vbar,
            width = width
        )?;
        Ok(())
    }

    fn context(&self, f: &mut dyn fmt::Write, diag: &(dyn Diagnostic)) -> fmt::Result {
        if let Some(src) = diag.context() {
            let charset = ThemeCharacters::unicode();

            let lineno = if let Some(loc) = diag.loc() {
                loc.get_line()
            } else {
                0
            };

            let context = if let Some(loc) = diag.loc() {
                format!("{}{} {}", charset.ltop, charset.hbar, loc)
            } else {
                format!("{}{} unknown source", charset.ltop, charset.hbar)
            };

            let ln_indent = lineno.to_string().len();
            writeln!(f, " {:width$} {}", " ", context, width = ln_indent)?;

            // Get the lines that we'll need to print.
            let mut lines = src.split('\n').collect::<Vec<&str>>();
            lines.retain(|x| !x.is_empty());
            let mut labels = if let Some(labels) = diag.labels() {
                labels.collect::<Vec<_>>()
            } else {
                std::vec!()
            };

            labels.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

            let mut lines_labels = Vec::new();

            let mut index = 0;
            for line in lines.into_iter() {
                let mut line_labels = Vec::new();
                let max_end = index + line.len();

                for label in &labels {
                    if label.start >= index && label.end <= max_end {
                        line_labels.push(label);
                    }
                }

                lines_labels.push((line, line_labels));
                index += line.len();
            }

            let mut lineno = lineno;
            for (line, mut labels) in lines_labels {
                self.write_linum(f, ln_indent, lineno, &charset)?;
                writeln!(f, "{}", line)?;
                self.render_labels(f, ln_indent, &mut labels, &charset, line.to_string())?;
                lineno += 1;
            }
            writeln!(
                f,
                " {:width$} {}{}",
                " ",
                charset.lbot,
                charset.hbar.to_string().repeat(3),
                width = ln_indent
            )?;
        } else {
            let context = if let Some(loc) = diag.loc() {
                format!("{}", loc)
            } else {
                "unknown source".into()
            };

            writeln!(f, "{}", context)?;
        }
        Ok(())
    }

    fn footer(&self, f: &mut dyn fmt::Write, diag: &(dyn Diagnostic)) -> fmt::Result {
        if let Some(h) = diag.help() {
            let help = format!("{}", h);

            let opts = textwrap::Options::new(80)
                .initial_indent("  ")
                .subsequent_indent("  ");

            let footer = format!("{}: {}", "help".bold(), help);
            writeln!(f, "{}", textwrap::fill(&footer, opts))?;
        }

        Ok(())
    }

    pub(crate) fn render_labels(
        &self,
        f: &mut dyn fmt::Write,
        ln_indent: usize,
        labels: &mut Vec<&LabeledSpan>,
        charset: &ThemeCharacters,
        content: String,
    ) -> fmt::Result {
        // @TODO: if the content starts with a tab the columns of labels are misplaced!
        // Pass the line as a parameter and replace every non-whitespace with a whitespace.
        let n = labels.len();
        let mut fline = String::new();

        let mut col = 1;
        for label in labels.iter().take(n) {
            if label.msg.is_some() {
                let len = label.end - label.start + 1;
                let middle: usize = len / 2;
                while col < label.start {
                    let c = content.chars().nth(col - 1).unwrap();
                    let c = if c.is_whitespace() { c } else { ' ' };
                    fline.push(c);
                    col += 1;
                }

                for i in 0..len {
                    if i != middle {
                        fline.push(charset.underline);
                        col += 1;
                    } else {
                        fline.push(charset.underbar);
                        col += 1;
                    }
                }
            }
        }
        self.write_no_linum(f, ln_indent, charset)?;
        writeln!(f, "{}", fline.red().bold())?;
        for line in 0..n {
            let mut label_line = String::new();
            let mut col = 1;

            for (i, label) in labels.iter().enumerate().take(n - line) {
                if let Some(msg) = &label.msg {
                    let len = label.end - label.start + 1;
                    let middle: usize = len / 2;

                    while col < label.start {
                        let c = content.chars().nth(col - 1).unwrap();
                        let c = if c.is_whitespace() { c } else { ' ' };
                        label_line.push(c);
                        col += 1;
                    }

                    if i != (n - line) - 1 {
                        for i in 0..len {
                            if i == middle {
                                label_line.push(charset.vbar);
                                col += 1;
                            } else {
                                let c = content.chars().nth(col - 1).unwrap();
                                let c = if c.is_whitespace() { c } else { ' ' };
                                label_line.push(c);
                                col += 1;
                            }
                        }
                    } else {
                        for i in 0..len {
                            if i == middle {
                                label_line.push(charset.lbot);
                                label_line.push(charset.hbar);
                                label_line.push(' ');
                                label_line.push_str(msg);
                                col += 3;
                            } else {
                                let c = content.chars().nth(col - 1);
                                let c = if let Some(c) = c {
                                    if c.is_whitespace() {
                                        c
                                    } else {
                                        ' '
                                    }
                                } else {
                                    ' '
                                };
                                label_line.push(c);
                                col += 1;
                            }
                        }
                    }
                }
            }
            self.write_no_linum(f, ln_indent, charset)?;
            writeln!(f, "{}", label_line.red().bold())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::diagnostic::err::LabeledSpan;

    use crate::source::loc::{DefaultLocation, Location};
    use std::fmt::Display;

    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("incompatible types")]
    struct FakeTypeError {}

    impl Diagnostic for FakeTypeError {
        fn severity(&self) -> Option<Severity> {
            Some(Severity::Error)
        }

        fn code<'a>(&self) -> Option<Box<dyn Display + 'a>> {
            Some(Box::new(std::any::type_name::<Self>()))
        }

        fn loc<'a>(&self) -> Option<Box<dyn Location + 'a>> {
            Some(Box::new(DefaultLocation {
                line: 10,
                col: 7,
                index: 0,
                source_name: "bad_file.l".into(),
            }))
        }

        fn context(&self) -> Option<String> {
            Some("x = 4 + \"this_is_a_str\";\nY = integrate(x^{e} - 0.23e+123123 * x - y)".into())
        }

        fn labels<'a>(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>> {
            let mut v = std::vec::Vec::new();
            v.push(LabeledSpan {
                msg: Some("int".into()),
                start: 5,
                end: 5,
            });
            v.push(LabeledSpan {
                msg: Some("cannot use operator '+' on these operands".into()),
                start: 7,
                end: 7,
            });
            v.push(LabeledSpan {
                msg: Some("string".into()),
                start: 9,
                end: 23,
            });

            v.push(LabeledSpan {
                msg: Some("string".into()),
                start: 24,
                end: 43,
            });

            Some(Box::new(v.into_iter()))
        }

        fn reason<'a>(&self) -> Option<Box<dyn Display + 'a>> {
            Some(Box::new("Operands must coerce to the same type."))
        }

        fn help<'a>(&self) -> Option<Box<dyn Display + 'a>> {
            Some(Box::new("Using '+' operator on a string and a non-string does not append the integer value to the string."))
        }
    }

    #[test]
    fn def_dman() {
        let mut dman = DefaultDiagnosticsManager {};
        let f = FakeTypeError {};
        dman.publish(Box::new(f));
    }
}
