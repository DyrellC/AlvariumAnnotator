use crate::Annotation;

pub trait Annotator {
    fn annotate(&mut self, data: &[u8]) -> crate::errors::Result<Annotation>;
}
