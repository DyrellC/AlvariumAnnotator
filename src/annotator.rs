use crate::Annotation;

pub trait Annotator {
    fn annotate(&self, data: &[u8]) -> Result<Annotation, String>;
}