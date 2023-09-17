# i3blocks-power

A battery monitor for i3blocks.

## Features
- Realtime status of battery level (e.g. 70% full)
- Realtime status of battery state (e.g. charging)

## Requirements
- i3blocks (running under i3 or sway)
- dbus installed and running
- upower installed and running

## Build
- Install rust
```bash
$ cargo build --release
$ install target/release/i3blocks-power /usr/local/bin/i3blocks-power
```

## Configuration
Add the following to `~/.config/i3blocks/config`:

```
[power]
command=$HOME/src/i3blocks-power/target/release/i3blocks-power
interval=persist
```

## TODO
- Power profile display and switching, similar to powerprofilesctl
