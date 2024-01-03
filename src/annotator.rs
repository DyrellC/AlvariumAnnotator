use crate::Annotation;

pub trait Annotator {
    fn annotate(&mut self, data: &[u8]) -> Result<Annotation, Box<dyn std::error::Error>>;
}
