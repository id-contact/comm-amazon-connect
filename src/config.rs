use id_contact_comm_common::config::Config as BaseConfig;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    base_config: BaseConfig,
    phonenumber: String,

    dtmf_length: usize,
    result_length: usize,
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
