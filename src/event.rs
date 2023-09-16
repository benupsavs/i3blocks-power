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

}