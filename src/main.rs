use std::{error, sync::{mpsc, Mutex, Arc}, thread, io::{BufReader, self, BufRead}};

use envconfig::Envconfig;
use i3blocks_power::{battery::UdevSubscription, config::Config, display::DisplayState, profiles::ProfileSubscription, event::ProfileState};

fn main() -> Result<(), Box<dyn error::Error>> {
    let _config = Config::init_from_env()?;
    let (tx, rx) = mpsc::channel();
    UdevSubscription::subscribe(tx.clone())?;
    let p_sub = ProfileSubscription::subscribe(tx)?;
    let p_tx = p_sub.tx;
    let current_state_r = Arc::new(Mutex::new(ProfileState::Unknown));
    let current_state_s = current_state_r.clone();
    thread::Builder::new().stack_size(16 * 1024).spawn(move || {
        let mut input = BufReader::new(io::stdin());
        let mut line = String::new();
        loop {
            line.clear();
            if let Ok(_) = input.read_line(&mut line) {
                if line.trim_end() == "1" {
                    if let Err(err) = p_tx.send_blocking(ProfileState::Low) {
                        eprintln!("error updating power profile: {:?}", err);
                        break;
                    }
                } else if line.trim_end() == "2" {
                    if let Err(err) = p_tx.send_blocking(ProfileState::Medium) {
                        eprintln!("error updating power profile: {:?}", err);
                        break;
                    }
                } else if line.trim_end() == "3" {
                    if let Err(err) = p_tx.send_blocking(ProfileState::High) {
                        eprintln!("error updating power profile: {:?}", err);
                        break;
                    }
                } else if line.trim_end() == "4" {
                    // Up
                    if let Ok(s) = current_state_r.lock() {
                        let new_state;
                        if *s == ProfileState::Low {
                            new_state = ProfileState::Medium;
                        } else if *s == ProfileState::Medium {
                            new_state = ProfileState::High;
                        } else {
                            continue;
                        }
                        if let Err(err) = p_tx.send_blocking(new_state) {
                            eprintln!("error updating power profile: {:?}", err);
                            break;
                        }
                    }
                } else if line.trim_end() == "5" {
                    // Down
                    if let Ok(s) = current_state_r.lock() {
                        let new_state;
                        if *s == ProfileState::Medium {
                            new_state = ProfileState::Low;
                        } else if *s == ProfileState::High {
                            new_state = ProfileState::Medium;
                        } else {
                            continue;
                        }
                        if let Err(err) = p_tx.send_blocking(new_state) {
                            eprintln!("error updating power profile: {:?}", err);
                            break;
                        }
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
                if let Ok(mut s) = current_state_s.lock() {
                    if pe.profile_name == "performance" {
                        *s = ProfileState::High;
                    } else if pe.profile_name == "balanced" {
                        *s = ProfileState::Medium;
                    } else if pe.profile_name == "power-saver" {
                        *s = ProfileState::Low;
                    }
                }
                disp.profile_event(&pe);
            },
        }
        println!("{}", disp.to_string());
    }
}
