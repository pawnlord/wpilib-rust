use crate::math::units::distance::{Feet, Meter};
use crate::math::units::time::Second;
use wpilib_macros::{unit, unit_conversion, unit_dimensional_analysis};
crate::crate_namespace!();

unit!(MeterPerSecond, f64);
unit!(KilometerPerHour, f64);
unit!(MilePerHour, f64);
unit!(FeetPerSecond, f64);

unit_conversion!(MeterPerSecond f64, KilometerPerHour f64, meter_per_second_to_kilometer_per_hour);
unit_conversion!(MeterPerSecond f64, MilePerHour f64, meter_per_second_to_mile_per_hour);
unit_conversion!(MeterPerSecond f64, FeetPerSecond f64, meter_per_second_to_feet_per_second);
unit_conversion!(FeetPerSecond f64, MilePerHour f64, feet_per_second_to_mile_per_hour);
unit_conversion!(FeetPerSecond f64, KilometerPerHour f64, feet_per_second_to_kilometer_per_hour);
unit_conversion!(MilePerHour f64, KilometerPerHour f64, mile_per_hour_to_kilometer_per_hour);

#[must_use]
pub fn meter_per_second_to_kilometer_per_hour(meter_per_second: f64) -> f64 {
    meter_per_second * 3.6
}

#[must_use]
pub fn meter_per_second_to_mile_per_hour(meter_per_second: f64) -> f64 {
    meter_per_second * 2.23694
}

#[must_use]
pub fn meter_per_second_to_feet_per_second(meter_per_second: f64) -> f64 {
    meter_per_second * 3.28084
}

#[must_use]
pub fn feet_per_second_to_mile_per_hour(feet_per_second: f64) -> f64 {
    meter_per_second_to_mile_per_hour(feet_per_second / 3.28084)
}

#[must_use]
pub fn feet_per_second_to_kilometer_per_hour(feet_per_second: f64) -> f64 {
    meter_per_second_to_kilometer_per_hour(feet_per_second / 3.28084)
}

#[must_use]
pub fn mile_per_hour_to_kilometer_per_hour(mile_per_hour: f64) -> f64 {
    meter_per_second_to_kilometer_per_hour(mile_per_hour / 2.23694)
}

unit_dimensional_analysis!(Meter / Second = MeterPerSecond);
unit_dimensional_analysis!(Feet / Second = FeetPerSecond);

impl MilePerHour {
    #[must_use]
    pub fn to_feet(&self, seconds: Second) -> Feet {
        Feet::new(FeetPerSecond::from(*self).value() * seconds.value())
    }
}

impl KilometerPerHour {
    #[must_use]
    pub fn to_meters(&self, seconds: Second) -> Meter {
        Meter::new(MeterPerSecond::from(*self).value() * seconds.value())
    }
}
