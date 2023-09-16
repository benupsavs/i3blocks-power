use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "BATTERY", default = "BAT0")]
    pub battery_name: String,
}