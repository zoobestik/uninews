use crate::cli::output::OutputConfig;
use crate::report::{ReportFactory, ReportMode, ReportStatus};
use console::style;
use std::time::Instant;

pub const INDENT: &str = "  ";
pub const TASK: &str = "→";
pub const SUCCESS: &str = "✓";
pub const FAIL: &str = "✗";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReportState {
    Pending,
    Finished,
    Failed,
    Skipped,
}

impl ReportState {
    fn is_terminal(&self) -> bool {
        !matches!(self, Self::Pending)
    }
}

pub struct Report {
    start: Instant,
    state: ReportState,
    mode: ReportMode,
    indent: usize,
}

fn print_task() -> String {
    match OutputConfig::get().colors_enabled {
        true => style(TASK).dim().to_string(),
        false => TASK.to_string(),
    }
}

impl Report {
    pub fn with_indent(&self, msg: String) -> String {
        format!("{}{}", INDENT.repeat(self.indent), msg)
    }

    fn start(&self) {
        match &self.mode {
            ReportMode::Complex(msg) => println!("{}", self.with_indent(format!("{}...", msg))),
            ReportMode::ComplexKid(msg) => println!(
                "{}",
                self.with_indent(format!("{} {}...", print_task(), msg))
            ),
            ReportMode::OneLiner(msg) => {
                print!("{}", self.with_indent(format!("{} {}", print_task(), msg)))
            }
            ReportMode::Silent => {}
        };
    }

    fn end_in_time(&self, msg: String) {
        let elapsed = self.start.elapsed();

        match &self.mode {
            ReportMode::Complex(name) | ReportMode::ComplexKid(name) => {
                println!(
                    "{}",
                    self.with_indent(format!("{} {} in {:.2}s", msg, name, elapsed.as_secs_f64()))
                )
            }
            ReportMode::Silent => {
                println!(
                    "{}",
                    self.with_indent(format!("{} in {:.2}s", msg, elapsed.as_secs_f64()))
                )
            }
            ReportMode::OneLiner(_) => println!(" in {:.2}s {}", elapsed.as_secs_f64(), msg),
        };
    }
}

impl ReportFactory for Report {
    fn make(mode: ReportMode, indent: usize) -> Self {
        let report = Self {
            start: Instant::now(),
            mode,
            indent,
            state: ReportState::Pending,
        };

        match &report.mode {
            ReportMode::Complex(_) | ReportMode::ComplexKid(_) | ReportMode::OneLiner(_) => {
                report.start()
            }
            ReportMode::Silent => {}
        }

        report
    }
}

impl ReportStatus for Report {
    fn indent(&self) -> usize {
        self.indent
    }

    fn skipped(&mut self) {
        self.skipped_with_text("");
    }

    fn skipped_with_text(&mut self, text: impl Into<String> + Send) {
        if self.state.is_terminal() {
            return;
        }

        let text = match text.into() {
            s if s.is_empty() => s,
            s => format!(" {}", s),
        };

        let msg = match OutputConfig::get().colors_enabled {
            true => style(" ...skipped").yellow().bold().to_string(),
            false => " ...skipped".to_string(),
        };

        match &self.mode {
            ReportMode::OneLiner(_) => println!("{}{}", text, msg),
            ReportMode::Complex(name) | ReportMode::ComplexKid(name) => {
                let text = match &text {
                    text if !text.is_empty() => text,
                    _ => name.as_str(),
                };
                println!("{}{}", msg, text)
            }
            ReportMode::Silent => println!("{} {}", msg, text),
        }
        self.state = ReportState::Skipped;
    }

    fn fail(&mut self) {
        if self.state.is_terminal() {
            return;
        }

        self.state = ReportState::Failed;
        self.end_in_time(match OutputConfig::get().colors_enabled {
            true => style(FAIL).red().bold().to_string(),
            false => FAIL.to_string(),
        });
    }

    fn finish(&mut self) {
        self.finish_with_text("");
    }

    fn finish_with_text(&mut self, text: impl Into<String> + Send) {
        if self.state.is_terminal() {
            return;
        }

        let text = match text.into() {
            s if s.is_empty() => s,
            s => format!(" {s}"),
        };

        self.state = ReportState::Finished;
        self.end_in_time(
            match OutputConfig::get().colors_enabled {
                true => style(SUCCESS).green().bold().to_string(),
                false => SUCCESS.to_string(),
            } + &text,
        );
    }
}
