use super::report::{INDENT, TASK};
use console::style;

pub fn display_error(error: &anyhow::Error) {
    eprintln!("{} {}", style("Error:").red().bold(), error);

    let mut source = error.source();

    while let Some(cause) = source {
        let prefix = INDENT;
        eprintln!("{}{} {}", prefix, style(TASK).dim(), style(cause).dim());
        source = cause.source();
    }
}
