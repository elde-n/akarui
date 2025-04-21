use clap::Parser;
use config::Config;

mod command;
mod config;
mod device;

fn main() {
    let cli = command::Command::parse();

    match &cli.command {
        command::Commands::List => {
            for device in device::list() {
                println!("{}", &device.name);
            }
        }

        command::Commands::Get { device } => {
            let device = device::find(device.clone());
            println!("{}", device.get_brightness());
        }

        command::Commands::Set { value, device } => {
            let device = device::find(device.clone());
            device.set_brightness(*value);
        }

        command::Commands::Increase { value, device } => {
            let device = device::find(device.clone());
            let brightness = device.get_brightness();
            device.set_brightness(brightness + value);
        }

        command::Commands::Decrease { value, device } => {
            let device = device::find(device.clone());
            let brightness = device.get_brightness();
            device.set_brightness(brightness - value);
        }

        command::Commands::Load => {
            let cfg: Config = confy::load(env!("CARGO_PKG_NAME"), Some("config")).unwrap();
            for device in device::list() {
                cfg.devices.iter().for_each(|x| {
                    if x.name == device.name {
                        device.set_brightness(x.brightness);
                    }
                })
            }
        }
    }
}
