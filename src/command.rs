use clap::{arg, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Command {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// lists available devices
    List,
    /// returns the backlight percentage of the device
    Get {
        #[arg(short, long)]
        device: Option<String>,
    },
    /// sets the backlight percentage of the device
    Set {
        value: u8,

        #[arg(short, long)]
        device: Option<String>,
    },

    Increase {
        value: u8,

        #[arg(short, long)]
        device: Option<String>,
    },

    Decrease {
        value: u8,

        #[arg(short, long)]
        device: Option<String>,
    },

    /// loads the stored backlight levels of all devices from the config file
    Load,
}
