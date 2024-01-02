use crate::constants::{self, AnnotationType, HashType};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Annotation {
    pub id: String,
    pub key: String,
    pub hash: HashType,
    pub host: String,
    pub kind: AnnotationType,
    pub signature: String,
    #[serde(rename = "isSatisfied")]
    pub is_satisfied: bool,
    pub timestamp: String,
}

#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AnnotationList {
    pub items: Vec<Annotation>,
}

impl Annotation {
    pub fn new(
        key: &str,
        hash: HashType,
        host: &str,
        kind: AnnotationType,
        is_satisfied: bool,
    ) -> Self {
        let timestamp = chrono::Local::now().to_rfc3339();
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

    pub fn validate_base(&self) -> bool {
        self.hash.is_base_hash_type() && self.kind.is_base_annotation_type()
    }
}

pub fn mock_annotation() -> Annotation {
    let key = "The hash of the contents";
    let hash = constants::SHA256_HASH.clone();
    let host = "Host Device";
    let kind = constants::ANNOTATION_SOURCE.clone();
    let satisfied = true;

    Annotation::new(key, hash, host, kind, satisfied)
}
