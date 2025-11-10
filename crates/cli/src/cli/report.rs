use crate::report::{Report as Reporter, ReportMode};
use console::{StyledObject, style};
use std::time::Instant;

pub const INDENT: &str = "  ";
pub const TASK: &str = "→";
pub const SUCCESS: &str = "✓";
pub const FAIL: &str = "✗";

pub struct Report {
    start: Instant,
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
            ReportMode::ComplexKid => format!("{} {}...", style(TASK).bold().dim(), msg),
            ReportMode::OneLiner => format!("{} {}", style(TASK).bold().dim(), msg),
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

impl Reporter for Report {
    fn indent(&self) -> usize {
        self.indent
    }

    fn make(name: impl Into<String>, mode: ReportMode, indent: usize) -> Self {
        let report = Self {
            start: Instant::now(),
            name: name.into(),
            mode,
            indent,
        };
        report.start(&report.name);
        report
    }

    fn skipped(&self) {
        let msg = style(" ...skipped").yellow().bold();

        println!(
            "{}",
            match self.mode {
                ReportMode::Complex | ReportMode::ComplexKid => {
                    format!("{}{} {}", self.indent_str(), self.name, msg,)
                }
                ReportMode::OneLiner => msg.to_string(),
            }
        )
    }

    fn fail(&self) {
        self.end_in_time(style(FAIL).red().bold())
    }

    fn finish(&self) {
        self.end_in_time(style(SUCCESS).green().bold())
    }
}
