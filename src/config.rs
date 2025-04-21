use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct Device {
    pub(crate) name: String,
    pub(crate) brightness: u8,
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) devices: Vec<Device>,
}
