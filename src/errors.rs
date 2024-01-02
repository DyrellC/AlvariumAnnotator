use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown annotation type")]
    UnknownAnnotation,
    #[error("annotation failed to serialize: {0}")]
    AnnotationSerialize(serde_json::Error),

    #[cfg(test)]
    #[error("provider failed to sign")]
    SignatureError,
    #[cfg(test)]
    #[error("provider failed to verify")]
    VerificationError,
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::AnnotationSerialize(err)
    }
}
