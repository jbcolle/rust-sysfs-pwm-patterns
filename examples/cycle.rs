extern crate sysfs_pwm_patterns;

use std::time::Duration;
use sysfs_pwm::Pwm;
use sysfs_pwm_patterns::rgbled::{PwmLedColour, RgbLed};
use sysfs_pwm_patterns::Pattern;

fn main() -> Result<(), anyhow::Error> {
    let red_pwm = Pwm::new(3, 0)?;
    let green_pwm = Pwm::new(2, 0)?;
    let blue_pwm = Pwm::new(0, 0)?;

    let rgbled = RgbLed::new(red_pwm, green_pwm, blue_pwm)?;

    let pattern = Pattern::Blink(Duration::from_millis(1000), PwmLedColour::RED);

    let mut pattern_handler = sysfs_pwm_patterns::PatternHandler::new(rgbled, pattern);
    pattern_handler.start();

    std::thread::sleep(Duration::from_secs(5));

    pattern_handler.set_pattern(&Pattern::BlinkTwice(
        Duration::from_millis(1000),
        PwmLedColour::new(255, 75, 0),
    ));

    std::thread::sleep(Duration::from_secs(5));

    pattern_handler.set_pattern(&Pattern::Breathe(
        Duration::from_millis(2500),
        PwmLedColour::BLUE,
    ));

    std::thread::sleep(Duration::from_secs(10));

    pattern_handler.set_pattern(&Pattern::BlinkBetweenColours(
        Duration::from_millis(1000),
        PwmLedColour::new(255, 75, 0),
        PwmLedColour::GREEN,
    ));

    std::thread::sleep(Duration::from_secs(5));

    Ok(())
}
