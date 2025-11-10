pub enum ReportMode {
    Complex,
    ComplexKid,
    OneLiner,
}

pub trait Report: Send + Sync {
    fn indent(&self) -> usize;

    fn make(name: impl Into<String>, mode: ReportMode, indent: usize) -> Self;
    fn new(name: impl Into<String>) -> Self
    where
        Self: Sized,
    {
        Self::make(name, ReportMode::Complex, 0)
    }

    fn simple(&self, name: impl Into<String>) -> Self
    where
        Self: Sized,
    {
        Self::make(name, ReportMode::OneLiner, self.indent() + 1)
    }

    fn complex(&self, name: impl Into<String>) -> Self
    where
        Self: Sized,
    {
        Self::make(name, ReportMode::ComplexKid, self.indent() + 1)
    }

    fn skipped(&self);
    #[allow(dead_code)] // @todo: remove
    fn fail(&self);
    fn finish(&self);
}
