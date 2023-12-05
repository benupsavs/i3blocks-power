use std::sync::mpsc;
use std::thread;

use async_channel::Sender;
use futures::StreamExt;
use futures::select;

use zbus::Connection;

use crate::event::{PowerEvent, ProfileEvent, ProfileState};
use crate::profiles_proxy::PowerProfilesProxy;

pub struct ProfileSubscription {
    pub tx: Sender<ProfileState>,
}

impl ProfileSubscription {
    pub fn subscribe(tx: mpsc::Sender<PowerEvent>) -> Result<ProfileSubscription, Box<dyn std::error::Error>> {
        let (p_tx, p_rx) = async_channel::unbounded::<ProfileState>();
        let _t = thread::Builder::new()
            .name("power profile subscription".into())
            .spawn(move || {
                futures::executor::block_on(async move {
                    let connection = Connection::system().await;
                    if let Err(err) = connection {
                        eprintln!("unable to connect to dbus: {:?}", err);
                        return;
                    }
                    let connection = connection.unwrap();
                    let pp = PowerProfilesProxy::new(&connection).await;
                    if let Err(err) = pp {
                        eprintln!("unable to connect to the power profiles proxy: {:?}", err);
                        return;
                    }
                    let pp = pp.unwrap();
                    let p = pp.active_profile().await;
                    if let Err(err) = p {
                        eprintln!("unable to fetch current power profile: {:?}", err);
                        return;
                    }
                    let p = p.unwrap();
                    if (tx.send(PowerEvent::Profile(ProfileEvent{profile_name: p}))).is_err() {
                        return;
                    }

                    let mut profile_stream = pp.receive_active_profile_changed().await.fuse();
                    let mut p_rxf = p_rx.fuse();
                    loop {
                        select! {
                            x = profile_stream.next() => {
                                if let Some(p) = x {
                                    if let Ok(p) = p.get().await {
                                        if (tx.send(PowerEvent::Profile(ProfileEvent{profile_name: p}))).is_err() {
                                            return;
                                        }
                                    }
                                }
                            },
                            x = p_rxf.next() => {
                                if let Some(ps) = x {
                                    // if let Err(err) = pp.set_active_profile(ps.name()).await {
                                    //     println!("error setting new profile: {:?}", err);
                                    // }
                                    // The above proxy code is not working. The following is a workaround.
                                    let pc = connection.call_method(
                                        Some("net.hadess.PowerProfiles"),
                                        "/net/hadess/PowerProfiles",
                                        Some("org.freedesktop.DBus.Properties"),
                                        "Set",
                                        &("net.hadess.PowerProfiles", "ActiveProfile", zbus::zvariant::Value::new(ps.name()))
                                    ).await;
                                    if let Err(err) = pc {
                                        eprintln!("error setting new profile: {:?}", err);
                                    }
                                }
                            },
                        }
                    }
                });
            });
        Ok(ProfileSubscription{tx: p_tx})
    }
}
