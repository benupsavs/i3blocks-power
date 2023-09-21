use crate::{battery::ChargingState, event::{BatteryEvent, ProfileEvent}};

#[derive(Default)]
pub struct DisplayState {
    pub bat_percentage: Option<f64>,
    pub charging_state: ChargingState,
    pub profile_name: &'static str,
}

impl ToString for DisplayState {
    fn to_string(&self) -> String {
        let mut line = String::new();
        if self.profile_name != "" {
            line.push_str(self.profile_name);
            line.push(':');
        }
        line.push_str(&self.charging_state.to_string());
        line.push_str(&self.bat_percentage.map(|p| p.to_string()).unwrap_or("?".to_string()));
        line.push('%');
        return line;
    }
}

impl DisplayState {
    pub fn new() -> DisplayState {
        DisplayState::default()
    }

    pub fn battery_event(&mut self, event: &BatteryEvent) {
        if let Some(percentage) = event.percentage {
            self.bat_percentage.replace(percentage);
        }
        if let Some(state) = event.state {
            self.charging_state = state;
        }
    }

    pub fn profile_event(&mut self, event: &ProfileEvent) {
        if event.profile_name == "performance" {
            self.profile_name = "H";
        } else if event.profile_name == "balanced" {
            self.profile_name = "M";
        } else if event.profile_name == "power-saver" {
            self.profile_name = "L";
        }
    }
}
