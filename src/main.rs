use std::{error, sync::mpsc, thread, io::{BufReader, self, BufRead}};

use envconfig::Envconfig;
use i3blocks_power::{battery::UdevSubscription, config::Config, display::DisplayState, profiles::ProfileSubscription};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::init_from_env()?;
    let (tx, rx) = mpsc::channel();
    UdevSubscription::subscribe(tx.clone())?;
    let p_sub = ProfileSubscription::subscribe(tx)?;
    let p_tx = p_sub.tx;
    thread::Builder::new().stack_size(16 * 1024).spawn(move || {
        let mut input = BufReader::new(io::stdin());
        let mut line = String::new();
        loop {
            line.clear();
            if let Ok(_) = input.read_line(&mut line) {
                if line.trim_end() == "1" {
                    if let Err(err) = p_tx.send_blocking(i3blocks_power::event::ProfileState::Low) {
                        eprintln!("error updating power profile: {:?}", err);
                        break;
                    }
                } else if line.trim_end() == "2" {
                    if let Err(err) = p_tx.send_blocking(i3blocks_power::event::ProfileState::Medium) {
                        eprintln!("error updating power profile: {:?}", err);
                        break;
                    }
                } else if line.trim_end() == "3" {
                    if let Err(err) = p_tx.send_blocking(i3blocks_power::event::ProfileState::High) {
                        eprintln!("error updating power profile: {:?}", err);
                        break;
                    }
                }
            } else {
                break;
            }
        }
    })?;
    let mut disp = DisplayState::new();
    loop {
        let e = rx.recv()?;
        match e {
            i3blocks_power::event::PowerEvent::Battery(be) => {
                disp.battery_event(&be);
            },
            i3blocks_power::event::PowerEvent::Profile(pe) => {
                disp.profile_event(&pe);
            },
        }
        println!("{}", disp.to_string());
    }
}
