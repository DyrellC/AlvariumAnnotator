mod annotator;
mod annotations;
mod providers;
mod config {
    pub trait StreamConfigWrapper {
        fn stream_type(&self) -> crate::constants::StreamType;
    }
}

mod error {
    use serde_json::Error;
    use thiserror::Error;
    #[derive(Error, Debug)]
    pub enum AnnotatorError {
        #[error("unknown annotation type")]
        UnknownAnnotation,
        #[error("annotation failed to serialize: {0}")]
        AnnotationSerialize(serde_json::Error),

        #[cfg(test)]
        #[error("provider failed to sign")]
        SignatureError,
        #[cfg(test)]
        #[error("provider failed to verify")]
        VerificationError
    }

    impl From<serde_json::Error> for AnnotatorError {
        fn from(err: Error) -> Self {
            AnnotatorError::AnnotationSerialize(err)
        }
    }
}


pub use annotator::Annotator;
pub use annotations::*;
pub use providers::*;
pub use config::StreamConfigWrapper;

pub mod constants;


