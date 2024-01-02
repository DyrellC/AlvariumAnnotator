use crate::errors::{Error, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

pub trait Validate {
    fn validate(&self) -> bool;
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct HashType(pub String);

lazy_static! {
    pub static ref MD5_HASH: HashType = HashType(String::from("md5"));
    pub static ref SHA256_HASH: HashType = HashType(String::from("sha256"));
    pub static ref NO_HASH: HashType = HashType(String::from("none"));
}

impl HashType {
    pub fn is_base_hash_type(&self) -> bool {
        self == MD5_HASH.deref() || self == SHA256_HASH.deref() || self == NO_HASH.deref()
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct KeyAlgorithm(pub String);

lazy_static! {
    pub static ref ED25519_KEY: KeyAlgorithm = KeyAlgorithm(String::from("ed25519"));
}

impl KeyAlgorithm {
    pub fn is_base_key_algorithm(&self) -> bool {
        self == ED25519_KEY.deref()
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct StreamType(pub String);

lazy_static! {
    pub static ref IOTA_STREAM: StreamType = StreamType(String::from("iota"));
    pub static ref MOCK_STREAM: StreamType = StreamType(String::from("mock"));
    pub static ref MQTT_STREAM: StreamType = StreamType(String::from("mqtt"));
}

impl StreamType {
    pub fn is_base_stream_type(&self) -> bool {
        self == IOTA_STREAM.deref() || self == MOCK_STREAM.deref() || self == MQTT_STREAM.deref()
    }
}
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationType(pub String);

lazy_static! {
    pub static ref ANNOTATION_PKI: AnnotationType = AnnotationType(String::from("pki"));
    pub static ref ANNOTATION_SOURCE: AnnotationType = AnnotationType(String::from("source"));
    pub static ref ANNOTATION_TLS: AnnotationType = AnnotationType(String::from("tls"));
    pub static ref ANNOTATION_TPM: AnnotationType = AnnotationType(String::from("tpm"));
}

impl AnnotationType {
    pub fn kind(&self) -> &str {
        &self.0
    }
    pub fn is_base_annotation_type(&self) -> bool {
        self == ANNOTATION_PKI.deref()
            || self == ANNOTATION_SOURCE.deref()
            || self == ANNOTATION_TLS.deref()
            || self == ANNOTATION_TPM.deref()
    }
}

impl TryFrom<&str> for AnnotationType {
    type Error = Error;
    fn try_from(kind: &str) -> Result<Self> {
        match kind {
            "source" => Ok(ANNOTATION_SOURCE.clone()),
            "pki" => Ok(ANNOTATION_PKI.clone()),
            "tls" => Ok(ANNOTATION_TLS.clone()),
            "tpm" => Ok(ANNOTATION_TPM.clone()),
            _ => Err(Error::UnknownAnnotation),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkAction(pub String);

lazy_static! {
    pub static ref ACTION_CREATE: SdkAction = SdkAction(String::from("create"));
    pub static ref ACTION_MUTATE: SdkAction = SdkAction(String::from("mutate"));
    pub static ref ACTION_TRANSIT: SdkAction = SdkAction(String::from("transit"));
    pub static ref ACTION_PUBLISH: SdkAction = SdkAction(String::from("publish"));
}

impl SdkAction {
    pub fn is_base_action(&self) -> bool {
        self == ACTION_CREATE.deref()
            || self == ACTION_MUTATE.deref()
            || self == ACTION_TRANSIT.deref()
            || self == ACTION_PUBLISH.deref()
    }
}
