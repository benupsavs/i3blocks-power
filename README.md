# i3blocks-power

A battery monitor blocklet for i3blocks. Runs in persistent mode and
receives events via upower/dbus, avoiding the need for polling or
spawning a script at a regular interval. If power-profiles-daemon
is running, power profile events are received, and low, medium, and high
profiles can be selected by using the mouse buttons.

## Features
- Realtime status of battery level (e.g. 70% full)
- Realtime status of battery state (e.g. charging)
- Realtime power profile state (L, M, or H)
- Changing power profile with mouse click

## Requirements
- i3blocks (running under i3 or sway)
- dbus installed and running
- upower installed and running

## Build
- Install rust
```bash
$ cargo build --release
$ sudo su -
# mkdir -p /usr/local/bin
# install target/release/i3blocks-power /usr/local/bin/i3blocks-power
```

## Configuration
Add the following to `~/.config/i3blocks/config`:

```
[power]
command=/usr/local/bin/i3blocks-power
interval=persist
```
