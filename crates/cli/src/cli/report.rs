use crate::report::{ReportFactory, ReportMode, ReportStatus};
use console::{StyledObject, style};
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

    name: String,
    mode: ReportMode,
    indent: usize,
}

impl Report {
    pub fn indent_str(&self) -> String {
        INDENT.repeat(self.indent)
    }

    fn start(&self, msg: &str) {
        let result = match self.mode {
            ReportMode::Complex => format!("{}...", msg),
            ReportMode::ComplexKid => format!("{} {}...", style(TASK).dim(), msg),
            ReportMode::OneLiner => format!("{} {}", style(TASK).dim(), msg),
        };

        match self.mode {
            ReportMode::Complex | ReportMode::ComplexKid => {
                println!("{}{}", self.indent_str(), result)
            }
            ReportMode::OneLiner => print!("{}{}", self.indent_str(), result),
        }
    }

    fn end_in_time(&self, msg: StyledObject<&str>) {
        let elapsed = self.start.elapsed();

        println!(
            "{}",
            match self.mode {
                ReportMode::Complex | ReportMode::ComplexKid => {
                    format!(
                        "{}{} {} in {:.2}s",
                        self.indent_str(),
                        msg,
                        self.name,
                        elapsed.as_secs_f64()
                    )
                }
                ReportMode::OneLiner => format!(" in {:.2}s {}", elapsed.as_secs_f64(), msg),
            }
        )
    }
}

impl ReportFactory for Report {
    fn make(name: impl Into<String>, mode: ReportMode, indent: usize) -> Self {
        let report = Self {
            start: Instant::now(),
            name: name.into(),
            mode,
            indent,
            state: ReportState::Pending,
        };
        report.start(&report.name);
        report
    }
}

impl ReportStatus for Report {
    fn indent(&self) -> usize {
        self.indent
    }

    fn skipped(&mut self) {
        if self.state.is_terminal() {
            return;
        }

        self.state = ReportState::Skipped;

        let msg = style(" ...skipped").yellow().bold();

        println!(
            "{}",
            match self.mode {
                ReportMode::Complex | ReportMode::ComplexKid => {
                    format!("{}{} {}", self.indent_str(), self.name, msg)
                }
                ReportMode::OneLiner => msg.to_string(),
            }
        );
    }

    fn fail(&mut self) {
        if self.state.is_terminal() {
            return;
        }

        self.state = ReportState::Failed;
        self.end_in_time(style(FAIL).red().bold());
    }

    fn finish(&mut self) {
        if self.state.is_terminal() {
            return;
        }

        self.state = ReportState::Finished;
        self.end_in_time(style(SUCCESS).green().bold());
    }
}
