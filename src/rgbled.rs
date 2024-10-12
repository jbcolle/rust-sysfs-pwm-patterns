use anyhow::{bail, Context, Error};
use sysfs_pwm::Pwm;

const DEFAULT_PERIOD_NS: u32 = 2000000;

#[derive(Clone, Copy)]
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

impl Colour {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn to_percentages(self) -> Vec<f32> {
        vec![
            self.red as f32 / 255.0,
            self.green as f32 / 255.0,
            self.blue as f32 / 255.0,
        ]
    }
}

pub struct RgbLed {
    pwms: [Pwm; 3],
    period_ns: u32,
    colour: Colour,
    brightness: f32,
}

impl RgbLed {
    pub fn new(pwm_r: Pwm, pwm_g: Pwm, pwm_b: Pwm) -> Result<Self, Error> {
        pwm_r.export()?;
        pwm_g.export()?;
        pwm_b.export()?;

        let pwms = [pwm_r, pwm_g, pwm_b];
        let mut rgbled = Self {
            pwms,
            period_ns: DEFAULT_PERIOD_NS,
            colour: Colour::new(0, 0, 0),
            brightness: 1.0,
        };

        rgbled.set_all_periods(DEFAULT_PERIOD_NS)?;

        Ok(rgbled)
    }

    pub fn set_colour_rgb(&mut self, colour: Colour) -> Result<(), Error> {
        self.colour = colour;

        self.set_colour_with_brightness(colour, self.brightness)
    }

    pub fn set_enable(&mut self, enable: bool) -> Result<(), Error> {
        self.pwms
            .iter()
            .try_for_each(|pwm| pwm.enable(enable))
            .context("Failed to enable PWMs")
    }

    pub fn set_brightness(&mut self, brightness: f32) -> Result<(), Error> {
        if !(0.0..=1.0).contains(&brightness) {
            bail!("Brightness {brightness} out of range. Expected 0..=1")
        }
        self.brightness = brightness;

        self.set_colour_with_brightness(self.colour, self.brightness)
    }

    fn set_colour_with_brightness(&mut self, colour: Colour, brightness: f32) -> Result<(), Error> {
        let mut duty_cycles = colour.to_percentages();
        duty_cycles.iter_mut().for_each(|ds| *ds *= brightness);

        for (index, duty_cycle) in duty_cycles.iter().enumerate() {
            self.set_pwm_duty_cycle_percent(*duty_cycle, index)?
        }

        Ok(())
    }

    fn set_all_periods(&mut self, period: u32) -> Result<(), Error> {
        self.pwms
            .iter()
            .try_for_each(|pwm| pwm.set_period_ns(period))
            .context("Couldn't set PWM periods")
    }

    fn set_pwm_duty_cycle_percent(
        &mut self,
        duty_cycle_pct: f32,
        pwm_index: usize,
    ) -> Result<(), Error> {
        if !(0.0..=1.0).contains(&duty_cycle_pct) {
            bail!("Expected duty cycle between 0.0 and 1.0")
        }

        let Some(pwm) = self.pwms.get(pwm_index) else {
            bail!("Could not get PWM with index {pwm_index}")
        };

        let duty_cycle_ns = duty_cycle_pct * (self.period_ns as f32);

        pwm.set_duty_cycle_ns(duty_cycle_ns as u32)?;

        Ok(())
    }
}

impl Drop for RgbLed {
    fn drop(&mut self) {
        self.set_enable(false).unwrap();
        self.pwms.iter().for_each(|pwm| pwm.unexport().unwrap());
    }
}

#[cfg(test)]
mod tests {}
