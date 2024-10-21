use crate::rgbled::{PwmLedColour, RgbLed};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;
use crate::Pattern::Full;

const DEFAULT_BREATHE_UPDATE_PERIOD_MS: u128 = 5;

pub mod rgbled;

#[derive(Debug, Clone)]
pub enum Pattern {
    Blink(Duration, PwmLedColour),
    BlinkTwice(Duration, PwmLedColour),
    BlinkBetweenColours(Duration, PwmLedColour, PwmLedColour),
    Breathe(Duration, PwmLedColour),
    BreatheBetweenColours(Duration, PwmLedColour, PwmLedColour),
    Full(PwmLedColour),
}

pub struct PatternHandler {
    driver: Arc<Mutex<RgbLed>>,
    pattern: Arc<Mutex<Pattern>>,
    is_running: bool,
    stop_flag: Arc<AtomicBool>,
}

impl PatternHandler {
    pub fn new(rgb_led: RgbLed, pattern: Pattern) -> Self {
        Self {
            driver: Arc::new(Mutex::new(rgb_led)),
            pattern: Arc::new(Mutex::new(pattern)),
            is_running: false,
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn set_pattern(&self, pattern: &Pattern) {
        if let Ok(mut current_pattern) = self.pattern.lock() {
            *current_pattern = pattern.clone();
        }
    }

    pub fn start(&mut self) {
        if self.is_running {
            println!("PatternHandler already running");
            return;
        } else {
            self.is_running = true;
            self.stop_flag.store(false, Ordering::SeqCst);
        }

        let pattern = self.pattern.clone();
        let led = self.driver.clone();
        let stop_flag = self.stop_flag.clone();

        thread::spawn(move || {
            let mut led = led.lock().expect("Could not get LED guard");
            led.set_brightness(0.0).unwrap();
            led.set_enable(true).unwrap();
            while !stop_flag.load(Ordering::SeqCst) {
                sleep(Duration::from_millis(1));
                let current_pattern = pattern.lock().expect("Could not get pattern guard").clone();
                match current_pattern {
                    Pattern::Blink(period, colour) => {
                        led.set_colour(colour).unwrap();
                        led.set_brightness(1.0).unwrap();
                        sleep(period / 2);
                        led.set_brightness(0.0).unwrap();
                        sleep(period / 2);
                    }
                    Pattern::BlinkTwice(period, colour) => {
                        led.set_colour(colour).unwrap();
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
                        led.set_colour(colour1).unwrap();
                        led.set_brightness(1.0).unwrap();
                        sleep(period / 2);
                        led.set_colour(colour2).unwrap();
                        sleep(period / 2)
                    }
                    Pattern::Breathe(period, colour) => {
                        led.set_colour(colour).unwrap();
                        let steps = period.as_millis() / DEFAULT_BREATHE_UPDATE_PERIOD_MS;
                        for i in 1..(steps / 2) {
                            led.set_brightness(i as f32 / (period.as_millis() as f32))
                                .unwrap();
                            sleep(Duration::from_millis(
                                DEFAULT_BREATHE_UPDATE_PERIOD_MS as u64,
                            ));
                        }
                        for i2 in (0..steps / 2).rev() {
                            led.set_brightness(i2 as f32 / (period.as_millis() as f32))
                                .unwrap();
                            sleep(Duration::from_millis(
                                DEFAULT_BREATHE_UPDATE_PERIOD_MS as u64,
                            ));
                        }
                    }
                    Pattern::Full(colour) => {
                        led.set_colour(colour).unwrap();
                        led.set_brightness(1.0).unwrap();
                    }
                    Pattern::BreatheBetweenColours(period, colour1, colour2) => {
                        led.set_colour(colour1).unwrap();
                        let steps = period.as_millis() / DEFAULT_BREATHE_UPDATE_PERIOD_MS;
                        for i in 1..(steps / 2) {
                            led.set_brightness(i as f32 / (period.as_millis() as f32))
                                .unwrap();
                            sleep(Duration::from_millis(
                                DEFAULT_BREATHE_UPDATE_PERIOD_MS as u64,
                            ));
                        }
                        led.set_colour(colour2).unwrap();
                        for i2 in (0..steps / 2).rev() {
                            led.set_brightness(i2 as f32 / (period.as_millis() as f32))
                                .unwrap();
                            sleep(Duration::from_millis(
                                DEFAULT_BREATHE_UPDATE_PERIOD_MS as u64,
                            ));
                        }
                    }
                }
            }
        });
    }

    pub fn stop(&self) {
        self.stop_flag.store(true, Ordering::SeqCst)
    }
}

impl Drop for PatternHandler {
    fn drop(&mut self) {
        self.set_pattern(&Full(PwmLedColour::new(0, 0, 0)));
        if self.is_running {
            self.stop();
        }
    }
}
