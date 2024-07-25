use crate::register_address::{ConfigBitFlags, TuningBitFlag};

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub config: u16,
    pub tuning: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            config: ConfigBitFlags::DHIZ
                | ConfigBitFlags::DMUTE
                | ConfigBitFlags::BASS
                | ConfigBitFlags::SEEKUP
                | ConfigBitFlags::RDS
                | ConfigBitFlags::NEW
                | ConfigBitFlags::ENABLE,
            tuning: TuningBitFlag::BAND_87_108_MHZ | TuningBitFlag::SPACE_100_KHZ,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    #[test]
    fn default_config() {
        assert_eq!(0b1101_0010_0001_1001, Config::default().config);
    }
}
