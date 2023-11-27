use crate::constants::{self, AnnotationType, HashType};
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Annotation<'a> {
    pub id: String,
    pub key: String,
    #[serde(borrow)]
    pub hash: HashType<'a>,
    pub host: String,
    pub kind: AnnotationType<'a>,
    pub signature: String,
    #[serde(rename = "isSatisfied")]
    pub is_satisfied: bool,
    pub timestamp: String,
}

#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AnnotationList<'a> {
    #[serde(borrow)]
    pub items: Vec<Annotation<'a>>
}

impl<'a> Annotation<'a> {
    pub fn new(key: &str, hash: HashType<'a>, host: &str, kind: AnnotationType<'a>, is_satisfied: bool) -> Self {
        let timestamp = chrono::Local::now().to_string();
        Annotation {
            id: ulid::Ulid::new().to_string(),
            key: key.to_string(),
            hash,
            host: host.to_string(),
            kind,
            signature: String::new(),
            is_satisfied,
            timestamp,
        }
    }

    pub fn with_signature(&mut self, signature: &str) {
        self.signature = signature.to_string()
    }

    pub fn validate(&self) -> bool {
        self.hash.validate() && self.kind.validate()
    }
}

pub fn mock_annotation<'a>() -> Annotation<'a> {
    let key = "The hash of the contents";
    let hash = constants::SHA256_HASH;
    let host = "Host Device";
    let kind = constants::ANNOTATION_SOURCE;
    let satisfied = true;

    Annotation::new(key, hash, host, kind, satisfied)
}