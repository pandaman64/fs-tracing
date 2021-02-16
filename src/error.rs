use std::{error, fmt, io};

#[derive(Debug)]
pub struct Error {
    source: io::Error,
    span: tracing_error::SpanTrace,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "source: {}\nspan:\n{}", self.source, self.span)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.source)
    }
}

impl Error {
    pub(crate) fn wrap_std(source: io::Error) -> io::Error {
        io::Error::new(
            source.kind(),
            Error {
                source,
                span: tracing_error::SpanTrace::capture(),
            },
        )
    }
}
