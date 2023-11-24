use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct HashType<'a>(pub &'a str);

pub const MD5_HASH: HashType = HashType("md5");
pub const SHA256_HASH: HashType = HashType("sha256");
pub const NO_HASH: HashType = HashType("none");

impl HashType<'_> {
    pub fn validate(&self) -> bool {
        self == &MD5_HASH || self == &SHA256_HASH || self == &NO_HASH
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct KeyAlgorithm<'a>(pub &'a str);

pub const ED25519_KEY: KeyAlgorithm = KeyAlgorithm("ed25519");

impl KeyAlgorithm<'_> {
    pub fn validate(&self) -> bool {
        self == &ED25519_KEY
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct StreamType<'a>(pub &'a str);

pub const IOTA_STREAM: StreamType = StreamType("iota");
pub const MOCK_STREAM: StreamType = StreamType("mock");
pub const MQTT_STREAM: StreamType = StreamType("mqtt");
pub const PRAVEGA_STREAM: StreamType = StreamType("pravega");

impl StreamType<'_> {
    pub fn validate(&self) -> bool {
        self == &IOTA_STREAM || self == &MOCK_STREAM || self == &MQTT_STREAM || self == &PRAVEGA_STREAM
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AnnotationType<'a>(pub &'a str);

pub const ANNOTATION_PKI: AnnotationType = AnnotationType("pki");
pub const ANNOTATION_SOURCE: AnnotationType = AnnotationType("source");
pub const ANNOTATION_TLS: AnnotationType = AnnotationType("tls");
pub const ANNOTATION_TPM: AnnotationType = AnnotationType("tpm");

impl AnnotationType<'_> {
    pub fn kind(&self) -> &str {
        self.0
    }
    pub fn validate(&self) -> bool {
        self == &ANNOTATION_PKI || self == &ANNOTATION_SOURCE || self == &ANNOTATION_TLS || self == &ANNOTATION_TPM
    }
}

impl TryFrom<&str> for AnnotationType<'_> {
    type Error = String;
    fn try_from(kind: &str) -> Result<Self, Self::Error> {
        match kind {
            "source" => Ok(ANNOTATION_SOURCE),
            "pki" => Ok(ANNOTATION_PKI),
            "tls" => Ok(ANNOTATION_TLS),
            "tpm" => Ok(ANNOTATION_TPM),
            _ => Err("unkown annotation type".to_string())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkAction<'a>(pub &'a str);

pub const ACTION_CREATE: SdkAction = SdkAction("create");
pub const ACTION_MUTATE: SdkAction = SdkAction("mutate");
pub const ACTION_TRANSIT: SdkAction = SdkAction("transit");
pub const ACTION_PUBLISH: SdkAction = SdkAction("publish");

impl SdkAction<'_> {
    pub fn validate(&self) -> bool {
        self == &ACTION_CREATE || self == &ACTION_MUTATE || self == &ACTION_TRANSIT || self == &ACTION_PUBLISH
    }
}