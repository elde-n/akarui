use std::path::{Path, PathBuf};

use crate::config::{self, Config};

const DEVICE_PATH: &str = "/sys/class/backlight/";

#[derive(Clone, Debug)]
pub(crate) struct Device {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
}

impl Device {
    fn new(path: &Path) -> Self {
        let name = path.file_name().unwrap();
        Self {
            name: name.to_string_lossy().to_string(),
            path: path.to_path_buf(),
        }
    }

    /// Set the brightness value as percentagee
    pub(crate) fn set_brightness(&self, value: u8) {
        let result = std::fs::read_to_string(self.path.join("max_brightness"));
        let max_brightness = result
            .expect("Failed to read max_brightness value")
            .trim()
            .parse::<f32>()
            .expect("Failed to parse max_brightness");

        let percentage = value.clamp(0, 100);
        let value = ((percentage as f32 / 100.0) * max_brightness).round() as u32;
        std::fs::write(self.path.join("brightness"), value.to_string())
            .expect("Failed to write brightness");

        let mut config: Config = confy::load(env!("CARGO_PKG_NAME"), Some("config")).unwrap();

        let device = config
            .devices
            .iter_mut()
            .filter(|x| x.name == self.name)
            .last();

        if let Some(device) = device {
            device.brightness = percentage as u8
        } else {
            config.devices.push(config::Device {
                name: self.name.clone(),
                brightness: percentage as u8,
            })
        }

        confy::store(env!("CARGO_PKG_NAME"), Some("config"), config)
            .expect("Failed to write backlight to config");
    }

    /// Returns the brightness value as percentage
    pub(crate) fn get_brightness(&self) -> u8 {
        let result = std::fs::read_to_string(self.path.join("brightness"));
        let brightness = result
            .expect("Failed to read brightness value")
            .trim()
            .parse::<f32>()
            .expect("Failed to parse brightness");

        let result = std::fs::read_to_string(self.path.join("max_brightness"));
        let max_brightness = result
            .expect("Failed to read max_brightness value")
            .trim()
            .parse::<f32>()
            .expect("Failed to parse max_brightness");

        ((brightness / max_brightness) * 100.0).round() as u8
    }
}

pub(crate) fn find(name: Option<String>) -> Device {
    let devices = list();
    let device = if name.is_none() {
        devices.first().expect("No devices found")
    } else {
        devices
            .iter()
            .map(|x| {
                if Some(x.name.clone()) == name {
                    Some(x)
                } else {
                    None
                }
            })
            .next()
            .expect("Device not found")
            .expect("Device not found")
    };

    device.clone()
}

pub(crate) fn list() -> Vec<Device> {
    let mut devices = Vec::new();
    for device in std::fs::read_dir(DEVICE_PATH).expect("Missing device path") {
        let link = device.unwrap();
        devices.push(Device::new(link.path().as_path()))
    }

    devices
}
