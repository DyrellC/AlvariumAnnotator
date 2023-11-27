pub mod hash_providers {

    pub trait HashProvider {
        fn derive(&self, data: &[u8]) -> String;
    }

    pub fn derive_hash<H: HashProvider>(hash_type: H, data: &[u8]) -> String {
        hash_type.derive(data)
    }

    #[cfg(test)]
    mod hash_provider_tests {
        use crate::hash_providers::HashProvider;

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

pub mod signature_provider {
    use crate::Annotation;

    pub trait SignProvider {
        fn sign(&self, key: &[u8], content: &[u8]) -> Result<String, Box<dyn std::error::Error>>;
        fn verify(&self, key: &[u8], content: &[u8], signed: &[u8]) -> Result<bool, Box<dyn std::error::Error>>;
    }

    pub fn serialise_and_sign<P, K>(provider: &P, key: K, annotation: &Annotation) -> Result<String, Box<dyn std::error::Error>>
        where
            P: SignProvider,
            K: AsRef<[u8]>,
    {
        let serialised = serde_json::to_vec(annotation)?;
        provider.sign(key.as_ref(), &serialised)
    }

    #[cfg(test)]
    mod annotation_utility_tests {
        use crate::annotations::mock_annotation;
        use crate::signature_provider::SignProvider;
        use super::serialise_and_sign;

        struct MockSignProvider {}

        impl SignProvider for MockSignProvider {
            fn sign(&self, key: &[u8], _content: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
                match key.eq("A known and correct key".as_bytes()) {
                    true => Ok("Signed".to_string()),
                    false => Err(Box::new(crate::error::AnnotatorError::SignatureError))
                }
            }

            fn verify(&self, key: &[u8], _content: &[u8], _signed: &[u8]) -> Result<bool, Box<dyn std::error::Error>> {
                match key.eq("A known and correct key".as_bytes()) {
                    true => Ok(true),
                    false => Err(Box::new(crate::error::AnnotatorError::VerificationError))
                }
            }
        }


        #[test]
        fn mock_sign_provider() {
            let mock_provider = MockSignProvider {};
            let annotation = mock_annotation();
            let correct_key = "A known and correct key".as_bytes();
            let unknown_key = "An unknown key".as_bytes();

            let failed_signature = serialise_and_sign(&mock_provider, unknown_key, &annotation);
            assert!(failed_signature.is_err());
            let signature = serialise_and_sign(&mock_provider, correct_key, &annotation);
            assert!(signature.is_ok());

            let ann_bytes = serde_json::to_vec(&annotation).unwrap();
            let failed_verify = mock_provider.verify(unknown_key, &ann_bytes, &ann_bytes);
            assert!(failed_verify.is_err());
            let verified = mock_provider.verify(correct_key, &ann_bytes, &ann_bytes);
            assert!(verified.is_ok())
        }
    }
}

pub mod stream_provider {
    use serde::{Serialize, Deserialize};
    use crate::constants::SdkAction;
    use crate::StreamConfigWrapper;


    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct MessageWrapper<'a>{
        #[serde(borrow)]
        pub action: SdkAction<'a>,
        #[serde(rename="messageType")]
        pub message_type: &'a str,
        pub content: &'a str,
    }


    #[async_trait::async_trait(?Send)]
    pub trait Publisher: Sized {
        type StreamConfig: StreamConfigWrapper;
        async fn new(cfg: &Self::StreamConfig) -> Result<Self, String>;
        async fn close(&mut self) -> Result<(), String>;
        async fn connect(&mut self) -> Result<(), String>;

        async fn reconnect(&mut self) -> Result<(), String>;
        async fn publish(&mut self, msg: MessageWrapper<'_>) -> Result<(), String>;
    }
}

