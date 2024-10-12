use crate::rgbled::{PwmLedColour, RgbLed};
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

const DEFAULT_PATTERN_DURATION_MS: Duration = Duration::from_millis(1000);
const DEFAULT_COLOUR: PwmLedColour = PwmLedColour::RED;

pub mod rgbled;

pub enum Pattern {
    Blink(Duration, PwmLedColour),
    BlinkTwice(Duration, PwmLedColour),
    BlinkBetweenColours(Duration, PwmLedColour, PwmLedColour),
    Breathe(Duration, PwmLedColour),
}

pub struct PatternHandler {
    driver: Arc<Mutex<RgbLed>>,
    pattern: Arc<Mutex<Pattern>>,
    is_running: Arc<Mutex<bool>>,
}

impl PatternHandler {
    pub fn new(rgb_led: RgbLed, pattern: Pattern) -> Self {
        Self {
            driver: Arc::new(Mutex::new(rgb_led)),
            pattern: Arc::new(Mutex::new(pattern)),
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn set_pattern(&self, pattern: Pattern) {
        if let Ok(mut current_pattern) = self.pattern.lock() {
            *current_pattern = pattern;
        }
    }

    pub fn start(&self) {
        if let Ok(mut is_running_guard) = self.is_running.lock() {
            if *is_running_guard {
                println!("PatternHandler already running");
                return;
            } else {
                *is_running_guard = true;
            }
        }

        let pattern = self.pattern.clone();
        let led = self.driver.clone();
        let is_running = self.is_running.clone();

        thread::spawn(move || {
            let mut led = led.lock().expect("Could not get LED guard");
            led.set_brightness(0.0).unwrap();
            led.set_enable(true).unwrap();
            loop {
                let current_pattern = pattern.lock().expect("Could not get pattern guard");

                match *current_pattern {
                    Pattern::Blink(period, colour) => {
                        led.set_colour_rgb(colour).unwrap();
                        led.set_brightness(1.0).unwrap();
                        sleep(period / 2);
                        led.set_brightness(0.0).unwrap();
                        sleep(period / 2);
                    }
                    Pattern::BlinkTwice(period, colour) => {
                        led.set_colour_rgb(colour).unwrap();
                        led.set_brightness(1.0).unwrap();
                        sleep(period / 8);
                        led.set_brightness(0.0).unwrap();
                        sleep(period / 8);
                        led.set_brightness(1.0).unwrap();
                        sleep(period / 8);
                        led.set_brightness(0.0).unwrap();
                        sleep(period * 5 / 8);
                    }
                    Pattern::BlinkBetweenColours(period, colour1, colour2) => {
                        led.set_colour_rgb(colour1).unwrap();
                        led.set_brightness(1.0).unwrap();
                        sleep(period / 2);
                        led.set_colour_rgb(colour2).unwrap();
                        sleep(period / 2)
                    }
                    Pattern::Breathe(period, colour) => {
                        for i in 1..=period.as_millis() {
                            led.set_brightness((i / period.as_millis()) as f32).unwrap()
                        }
                        for i in (0..(period.as_millis() - 1)).rev() {
                            led.set_brightness((i / period.as_millis()) as f32).unwrap()
                        }
                    }
                }
            }
        });
    }
}
