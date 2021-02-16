use std::{error, fmt, io};

#[derive(Debug)]
pub struct Error {
    message: String,
    source: Option<Box<dyn error::Error + Send + Sync + 'static>>,
    span: tracing_error::SpanTrace,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // CR pandaman: more nice message?
        writeln!(f, "{}", self.message)?;

        if let Some(source) = &self.source {
            writeln!(f, "{}", source)?;
        }

        write!(f, "Trace:\n{}", self.span)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.source {
            Some(source) => {
                let source: &(dyn error::Error + 'static) = &**source;
                Some(source)
            }
            None => None,
        }
    }
}

impl Error {
    pub(crate) fn wrap_std(source: io::Error) -> io::Error {
        let kind = source.kind();
        let message = source.to_string();
        let source = source.into_inner();

        io::Error::new(
            kind,
            Error {
                message,
                source,
                span: tracing_error::SpanTrace::capture(),
            },
        )
    }
}
