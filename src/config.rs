use crate::error::Error;

use id_contact_jwt::{EncryptionKeyConfig, SignKeyConfig};
use josekit::{jwe::JweDecrypter, jws::JwsVerifier};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Deserialize;

use std::convert::TryFrom;

#[derive(Deserialize, Debug)]
struct RawConfig {
    internal_url: String,
    phonenumber: String,

    dtmf_length: usize,
    result_length: usize,

    decryption_privkey: EncryptionKeyConfig,
    signature_pubkey: SignKeyConfig,
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "RawConfig")]
pub struct Config {
    internal_url: String,
    phonenumber: String,

    dtmf_length: usize,
    result_length: usize,

    decrypter: Box<dyn JweDecrypter>,
    validator: Box<dyn JwsVerifier>,
}

// This tryfrom can be removed once try_from for fields lands in serde
impl TryFrom<RawConfig> for Config {
    type Error = Error;
    fn try_from(config: RawConfig) -> Result<Config, Error> {
        Ok(Config {
            internal_url: config.internal_url,
            phonenumber: config.phonenumber,

            dtmf_length: config.dtmf_length,
            result_length: config.result_length,

            decrypter: Box::<dyn JweDecrypter>::try_from(config.decryption_privkey)?,
            validator: Box::<dyn JwsVerifier>::try_from(config.signature_pubkey)?,
        })
    }
}

impl Config {
    pub fn generate_dtmf(&self) -> String {
        const NUMERIC: &[u8] = b"0123456789";
        let mut rng = thread_rng();
        (0..self.dtmf_length)
            .map(|_| {
                let idx = rng.gen_range(0..NUMERIC.len());
                NUMERIC[idx] as char
            })
            .collect()
    }

    pub fn generate_resultcode(&self) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(self.result_length)
            .map(char::from)
            .collect()
    }

    pub fn decrypter(&self) -> &dyn JweDecrypter {
        self.decrypter.as_ref()
    }

    pub fn validator(&self) -> &dyn JwsVerifier {
        self.validator.as_ref()
    }

    pub fn phonenumber(&self) -> &str {
        &self.phonenumber
    }

    pub fn internal_url(&self) -> &str {
        &self.internal_url
    }
}
