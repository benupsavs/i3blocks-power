use std::{sync::mpsc::Sender, thread};

use futures::StreamExt;
use futures::select;
use upower_dbus::UPowerProxy;
use upower_dbus::BatteryState;

use crate::event::BatteryEvent;
use crate::event::PowerEvent;

pub struct UdevSubscription {}

impl UdevSubscription {
    /// creates a battery event subscription using udev.
    pub fn subscribe(tx: Sender<PowerEvent>) -> Result<UdevSubscription, Box<dyn std::error::Error>> {
        let t = thread::Builder::new()
            .name("upower subscription".into())
            .spawn(move || {
                futures::executor::block_on(async move {
                    let connection = zbus::Connection::system().await;
                    if let Err(err) = connection {
                        eprintln!("unable to connect to dbus: {:?}", err);
                        return;
                    }
                    let connection = connection.unwrap();
                    let upower = UPowerProxy::new(&connection).await;
                    if let Err(err) = upower {
                        eprintln!("unable to connect to the upower proxy: {:?}", err);
                        return;
                    }
                    let upower = upower.unwrap();
                    let device = upower.get_display_device().await;
                    if let Err(err) = device {
                        eprintln!("no battery found: {:?}", err);
                        return;
                    }
                    let device = device.unwrap();
                    let percentage = device.percentage().await.unwrap_or_default();
                    let battery_state = device.state().await.unwrap_or(BatteryState::Unknown);
                    // println!("Battery: {}, {:?}", device.percentage().await.map(|p| p.to_string()).unwrap_or("Unknown".to_string()), res);
                    if tx.send(PowerEvent::Battery(BatteryEvent{percentage: Some(percentage), state: Some(ChargingState(battery_state))})).is_err() {
                        return;
                    }

                    let mut state_stream = device.receive_state_changed().await.fuse();
                    let mut percent_stream = device.receive_percentage_changed().await.fuse();
                    loop {
                        select! {
                            x = state_stream.next() => {
                                if x.is_some() {
                                    if let Ok(s) = device.state().await {
                                        if tx.send(PowerEvent::Battery(BatteryEvent{percentage: None, state: Some(ChargingState(s))})).is_err() {
                                            return;
                                        }
                                    }
                                }
                            },
                            x = percent_stream.next() => {
                                if x.is_some() {
                                    if let Ok(p) = device.percentage().await {
                                        if tx.send(PowerEvent::Battery(BatteryEvent{percentage: Some(p), state: None})).is_err() {
                                            return;
                                        }
                                    }
                                }
                            }
                        };
                    }
                    // while let Some(v) = state_stream.next().await {
                    //     println!("Got: {:?}", v.get().await);
                    // }
                });
            })?;
        return Ok(UdevSubscription {})
    }
}

#[derive(Copy, Clone)]
pub struct ChargingState(upower_dbus::BatteryState);

impl ToString for ChargingState {
    fn to_string(&self) -> String {
        match self.0 {
            BatteryState::Unknown => "?".into(),
            BatteryState::Charging => ">".into(),
            BatteryState::Discharging => "<".into(),
            BatteryState::Empty => "".into(),
            BatteryState::FullyCharged => "C".into(),
            BatteryState::PendingCharge => "P>".into(),
            BatteryState::PendingDischarge => "P<".into(),
        }
    }
}

impl Default for ChargingState {
    fn default() -> Self {
        ChargingState(BatteryState::Unknown)
    }
}