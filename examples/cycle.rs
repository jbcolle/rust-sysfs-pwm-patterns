extern crate sysfs_pwm_patterns;

use sysfs_pwm::Pwm;
use sysfs_pwm_patterns::RgbLed;

fn main() -> Result<(), anyhow::Error> {
    let red_pwm = Pwm::new(4, 0)?;
    let green_pwm = Pwm::new(3, 0)?;
    let blue_pwm = Pwm::new(0, 0)?;

    let mut rgbled = RgbLed::new(red_pwm, green_pwm, blue_pwm)?;

    let col = sysfs_pwm_patterns::Colour::new(255, 60, 0);
    rgbled.set_colour_rgb(col)?;
    rgbled.set_enable(true)?;

    for _ in 0..5{
        for val in (0..=50).rev() {
            rgbled.set_brightness(val as f32 / 50.0)?;
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        for val in (0..=50) {
            rgbled.set_brightness(val as f32 / 50.0)?;
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    Ok(())
}
