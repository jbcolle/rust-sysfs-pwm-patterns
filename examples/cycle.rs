extern crate sysfs_pwm_patterns;

use std::time::Duration;
use sysfs_pwm::Pwm;
use sysfs_pwm_patterns::Pattern;
use sysfs_pwm_patterns::rgbled::{PwmLedColour, RgbLed};

fn main() -> Result<(), anyhow::Error> {
    let red_pwm = Pwm::new(4, 0)?;
    let green_pwm = Pwm::new(3, 0)?;
    let blue_pwm = Pwm::new(0, 0)?;

    let rgbled = RgbLed::new(red_pwm, green_pwm, blue_pwm)?;

    let pattern = sysfs_pwm_patterns::Pattern::Blink(Duration::from_millis(1000), PwmLedColour::RED);
    
    let pattern_handler = sysfs_pwm_patterns::PatternHandler::new(rgbled, pattern);
    
    std::thread::sleep(Duration::from_secs(5));
    
    pattern_handler.set_pattern(Pattern::Breathe(Duration::from_millis(2000), PwmLedColour::GREEN));
    
    std::thread::sleep(Duration::from_secs(5));
    
    Ok(())
}
