use miette::Report;
use tracing::error;

pub trait TraceReport {
    fn trace_report(self) -> Self;
}

impl TraceReport for Report {
    fn trace_report(self) -> Self {
        error!(report = %self);
        eprintln!("{self:?}");
        self
    }
}

impl<T> TraceReport for Result<T, Report> {
    fn trace_report(self) -> Self {
        self.inspect_err(|report| {
            error!(%report);
            eprintln!("{report:?}");
        })
    }
}
