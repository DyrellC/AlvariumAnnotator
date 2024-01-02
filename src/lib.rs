mod annotations;
mod annotator;
mod errors;
mod providers;
mod config {
    pub trait StreamConfigWrapper {
        fn stream_type(&self) -> &crate::constants::StreamType;
    }
}

pub use annotations::*;
pub use annotator::Annotator;
pub use config::StreamConfigWrapper;
pub use errors::Error;
pub use providers::*;

pub mod constants;
