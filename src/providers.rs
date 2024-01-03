mod hash_providers {

    pub trait HashProvider {
        fn derive(&self, data: &[u8]) -> String;
    }

    pub fn derive_hash<H: HashProvider>(hash_type: H, data: &[u8]) -> String {
        hash_type.derive(data)
    }

    #[cfg(test)]
    mod hash_provider_tests {
        use crate::HashProvider;

        struct MockHashProvider {}

        impl HashProvider for MockHashProvider {
            fn derive(&self, _data: &[u8]) -> String {
                "Derived".to_string()
            }
        }

        #[test]
        fn test_mock_derive() {
            let hash_provider = MockHashProvider {};
            let derived = hash_provider.derive("data".as_bytes());
            assert_eq!("Derived", derived)
        }
    }
}

mod signature_provider {
    use crate::Annotation;

    pub trait SignProvider {
        type Error: std::error::Error;
        fn sign(&self, content: &[u8]) -> Result<String, Self::Error>;
        fn verify(&self, content: &[u8], signed: &[u8]) -> Result<bool, Self::Error>;
    }

    pub fn serialise_and_sign<P>(provider: &P, annotation: &Annotation) -> Result<String, P::Error>
    where
        P: SignProvider,
        <P as SignProvider>::Error: From<serde_json::Error>
    {
        let serialised = serde_json::to_vec(annotation)?;
        provider.sign(&serialised)
    }

    #[cfg(test)]
    mod annotation_utility_tests {
        use super::serialise_and_sign;
        use crate::annotations::mock_annotation;
        use crate::errors::{Error, Result};
        use crate::SignProvider;

        struct MockSignProvider {
            pub public: String,
            pub private: String,
        }

        impl SignProvider for MockSignProvider {
            type Error = crate::errors::Error;
            fn sign(&self, _content: &[u8]) -> Result<String> {
                match self.private.as_str().eq("A known and correct key") {
                    true => Ok("Signed".to_string()),
                    false => Err(Error::SignatureError),
                }
            }

            fn verify(&self, _content: &[u8], _signed: &[u8]) -> Result<bool> {
                match self.public.as_str().eq("A known and correct key") {
                    true => Ok(true),
                    false => Err(Error::VerificationError),
                }
            }
        }

        #[test]
        fn mock_sign_provider() {
            let correct_key = "A known and correct key".to_string();
            let unknown_key = "An unknown key".to_string();

            let mock_provider = MockSignProvider {
                private: correct_key.clone(),
                public: correct_key.clone(),
            };

            let bad_mock_provider = MockSignProvider {
                private: unknown_key.clone(),
                public: unknown_key.clone(),
            };

            let annotation = mock_annotation();

            let failed_signature = serialise_and_sign(&bad_mock_provider, &annotation);
            assert!(failed_signature.is_err());
            let signature = serialise_and_sign(&mock_provider, &annotation);
            assert!(signature.is_ok());

            let ann_bytes = serde_json::to_vec(&annotation).unwrap();
            let failed_verify = bad_mock_provider.verify("Content".as_bytes(), &ann_bytes);
            assert!(failed_verify.is_err());
            let verified = mock_provider.verify("Content".as_bytes(), &ann_bytes);
            assert!(verified.is_ok())
        }
    }
}

mod stream_provider {
    use crate::constants::SdkAction;
    use crate::StreamConfigWrapper;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct MessageWrapper<'a> {
        pub action: SdkAction,
        #[serde(rename = "messageType")]
        pub message_type: &'a str,
        pub content: &'a str,
    }

    #[async_trait::async_trait]
    pub trait Publisher: Sized {
        type StreamConfig: StreamConfigWrapper;
        type Error: std::error::Error;
        async fn new(cfg: &Self::StreamConfig) -> Result<Self, Self::Error>;
        async fn close(&mut self) -> Result<(), Self::Error>;
        async fn connect(&mut self) -> Result<(), Self::Error>;
        async fn reconnect(&mut self) -> Result<(), Self::Error>;
        async fn publish(&mut self, msg: MessageWrapper<'_>) -> Result<(), Self::Error>;
    }
}

pub use hash_providers::{derive_hash, HashProvider};
pub use signature_provider::{serialise_and_sign, SignProvider};
pub use stream_provider::{MessageWrapper, Publisher};
