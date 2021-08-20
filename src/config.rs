use id_contact_comm_common::config::{RawConfig as RawBaseConfig, Config as BaseConfig};
use id_contact_comm_common::error::Error;
use id_contact_jwt::{EncryptionKeyConfig, SignKeyConfig};
use josekit::{jwe::JweDecrypter, jws::JwsVerifier};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Deserialize;

use std::convert::TryFrom;

#[derive(Deserialize, Debug)]
struct RawConfig {
    #[serde(flatten)]
    raw_base_config: RawBaseConfig,
    phonenumber: String,

    dtmf_length: usize,
    result_length: usize,
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "RawConfig")]
pub struct Config {
    base_config: BaseConfig,
    phonenumber: String,

    dtmf_length: usize,
    result_length: usize,
}

// This tryfrom can be removed once try_from for fields lands in serde
impl TryFrom<RawConfig> for Config {
    type Error = Error;
    fn try_from(config: RawConfig) -> Result<Config, Error> {
        Ok(Config {
            base_config: BaseConfig::try_from(config.raw_base_config)?,
            phonenumber: config.phonenumber,

            dtmf_length: config.dtmf_length,
            result_length: config.result_length,
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

    pub fn phonenumber(&self) -> &str {
        &self.phonenumber
    }

    pub fn base_config(&self) -> &BaseConfig {
        &self.base_config
    }
}
