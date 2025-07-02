use miette::Result;
use tracing::error;

pub trait TraceReport<T> {
    fn trace_report(self) -> Result<T>;
}

impl<T> TraceReport<T> for Result<T> {
    #[inline(always)]
    fn trace_report(self) -> Result<T> {
        self.inspect_err(|report| {
            error!(%report);
            eprintln!("{report:?}");
        })
    }
}
