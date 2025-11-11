use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;

pub enum ReportMode {
    Complex,
    ComplexKid,
    OneLiner,
}

pub type ReportFuture<'a, T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'a>>;

pub trait ReportFactory: Sized {
    fn make(name: impl Into<String>, mode: ReportMode, indent: usize) -> Self;
}

pub trait ReportStatus {
    fn indent(&self) -> usize;
    fn skipped(&mut self);
    fn fail(&mut self);
    fn finish(&mut self);
}

pub trait Report: ReportFactory + ReportStatus + Send + Sync + 'static {}

impl<T> Report for T where T: ReportFactory + ReportStatus + Send + Sync + 'static {}

trait ReportHelper: Report {
    async fn run_with_reporting<F, T, E>(mut task: Self, f: F) -> Result<T, E>
    where
        F: for<'a> FnOnce(&'a mut Self) -> ReportFuture<'a, T, E> + Send,
        T: Send,
    {
        let result = f(&mut task).await;

        match result {
            Ok(_) => task.finish(),
            Err(_) => task.fail(),
        }

        result
    }
}

impl<T: Report> ReportHelper for T {}

#[async_trait]
pub trait ReportExt: Report {
    async fn task<F, T, E>(name: impl Into<String> + Send, f: F) -> Result<T, E>
    where
        F: for<'a> FnOnce(&'a mut Self) -> ReportFuture<'a, T, E> + Send,
        T: Send,
    {
        Self::run_with_reporting(Self::make(name, ReportMode::Complex, 0), f).await
    }

    async fn simple<F, T, E>(&self, name: impl Into<String> + Send, f: F) -> Result<T, E>
    where
        F: for<'a> FnOnce(&'a mut Self) -> ReportFuture<'a, T, E> + Send,
        T: Send,
    {
        Self::run_with_reporting(Self::make(name, ReportMode::OneLiner, self.indent() + 1), f).await
    }

    async fn complex<F, T, E>(&self, name: impl Into<String> + Send, f: F) -> Result<T, E>
    where
        F: for<'a> FnOnce(&'a mut Self) -> ReportFuture<'a, T, E> + Send,
        T: Send,
    {
        Self::run_with_reporting(
            Self::make(name, ReportMode::ComplexKid, self.indent() + 1),
            f,
        )
        .await
    }
}

impl<T: Report> ReportExt for T {}
