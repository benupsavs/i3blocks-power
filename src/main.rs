use std::{error, sync::mpsc};

use envconfig::Envconfig;
use i3blocks_power::{battery::UdevSubscription, config::Config, display::DisplayState};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::init_from_env()?;
    let (tx, rx) = mpsc::channel();
    UdevSubscription::subscribe(tx)?;
    let mut disp = DisplayState::new();
    loop {
        let e = rx.recv()?;
        match e {
            i3blocks_power::event::PowerEvent::Battery(be) => {
                disp.battery_event(&be);
            },
            i3blocks_power::event::PowerEvent::Profile(_pe) => {
                todo!();
            },
        }
        println!("{}", disp.to_string());
    }
}
