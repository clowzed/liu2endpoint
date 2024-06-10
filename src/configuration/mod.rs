pub use env::*;
pub use reader::*;
use serde::{Deserialize, Serialize};
use url::Url;

pub mod env;
pub mod reader;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Configuration {
    imap_host: String,
    imap_port: u16,
    imap_login: String,
    imap_password: String,
    imap_folder: String,
    resend_to: Url,
    recheck_interval: u64,
}

impl Configuration {
    pub fn imap_host(&self) -> &str {
        &self.imap_host
    }

    pub fn imap_port(&self) -> u16 {
        self.imap_port
    }

    pub fn imap_login(&self) -> &str {
        &self.imap_login
    }

    pub fn imap_password(&self) -> &str {
        &self.imap_password
    }

    pub fn imap_folder(&self) -> &str {
        &self.imap_folder
    }

    pub fn resend_to(&self) -> &Url {
        &self.resend_to
    }

    pub fn recheck_interval(&self) -> u64 {
        self.recheck_interval
    }
}
