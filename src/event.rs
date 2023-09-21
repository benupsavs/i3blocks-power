use crate::battery::ChargingState;

/// Container for one of the possible power events.
pub enum PowerEvent {
    Battery(BatteryEvent),
    Profile(ProfileEvent),
}

/// Event sent when a monitored battery state changes.
/// Each field should be separately checked for presence of a value.
pub struct BatteryEvent {
    pub percentage: Option<f64>,
    pub state: Option<ChargingState>,
}

/// Event sent when a power profile changes.
pub struct ProfileEvent {
    pub profile_name: String,
}

// One of the power profile states.
pub enum ProfileState {
    Low,
    Medium,
    High,
}

impl ProfileState {
    pub fn name(&self) -> &'static str {
        match self {
            ProfileState::Low => "power-saver",
            ProfileState::Medium => "balanced",
            ProfileState::High => "performance",
        }
    }
}