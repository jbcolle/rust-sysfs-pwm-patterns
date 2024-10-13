use crate::rgbled::{PwmLedColour, RgbLed};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

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

    pub fn set_pattern(&self, pattern: Pattern) {
        if let Ok(mut current_pattern) = self.pattern.lock() {
            *current_pattern = pattern;
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
                        led.set_colour_rgb(colour).unwrap();
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

    pub fn stop(&self) {
        self.stop_flag.store(true, Ordering::SeqCst)
    }
}
