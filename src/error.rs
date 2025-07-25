use miette::Report;
use tracing::error;

pub trait TraceReport {
    fn trace(self) -> Self;
    fn trace_report(self) -> Self;
}

impl TraceReport for &Report {
    #[track_caller]
    #[inline]
    fn trace(self) -> Self {
        let location = std::panic::Location::caller();
        let at = format!(
            "{}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
        error!(report = %self, %at);
        self
    }

    #[track_caller]
    #[inline]
    fn trace_report(self) -> Self {
        self.trace();
        eprintln!("{self:?}");
        self
    }
}

impl<T> TraceReport for Result<T, Report> {
    #[track_caller]
    #[inline]
    fn trace(self) -> Self {
        {
            if let Err(e) = &self {
                e.trace();
            }
            self
        }
    }

    #[track_caller]
    #[inline]
    fn trace_report(self) -> Self {
        {
            if let Err(e) = &self {
                e.trace_report();
            }
            self
        }
    }
}
