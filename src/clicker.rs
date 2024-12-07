use crate::mouse_button::SerializableMouseButton;
use enigo::{Enigo, Mouse, Settings};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClickerConfig {
    pub click_interval_ms: u64,
    pub mouse_button: SerializableMouseButton,
    pub random_delay_enabled: bool,
    pub random_delay_min_ms: u64,
    pub random_delay_max_ms: u64,
}

impl Default for ClickerConfig {
    fn default() -> Self {
        Self {
            click_interval_ms: 1000,
            mouse_button: SerializableMouseButton::Left,
            random_delay_enabled: false,
            random_delay_min_ms: 0,
            random_delay_max_ms: 500,
        }
    }
}

#[derive(Debug)]
pub struct Clicker {
    pub config: ClickerConfig,
    is_clicking: Arc<AtomicBool>,
    click_count: Arc<AtomicU64>,
}

impl Default for Clicker {
    fn default() -> Self {
        Self::new(ClickerConfig::default())
    }
}

impl Clicker {
    pub fn new(config: ClickerConfig) -> Self {
        Self {
            config,
            is_clicking: Arc::new(AtomicBool::new(false)),
            click_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn start_clicking(&mut self) {
        if !self.is_clicking.load(Ordering::SeqCst) {
            self.is_clicking.store(true, Ordering::SeqCst);
            let is_clicking = Arc::clone(&self.is_clicking);
            let click_count = Arc::clone(&self.click_count);
            let config = self.config.clone();

            thread::spawn(move || {
                let settings = Settings::default();
                let mut enigo = Enigo::new(&settings).expect("Failed to create Enigo instance");
                let mut rng = rand::thread_rng();

                while is_clicking.load(Ordering::SeqCst) {
                    let mouse_button = config.mouse_button;
                    if let Err(e) = enigo.button(mouse_button.into(), enigo::Direction::Click) {
                        eprintln!("Failed to click mouse button: {}", e);
                    }
                    click_count.fetch_add(1, Ordering::SeqCst);

                    let delay = if config.random_delay_enabled {
                        config.click_interval_ms
                            + rng.gen_range(config.random_delay_min_ms..=config.random_delay_max_ms)
                    } else {
                        config.click_interval_ms
                    };

                    thread::sleep(Duration::from_millis(delay));
                }
            });
        }
    }

    pub fn stop_clicking(&mut self) {
        self.is_clicking.store(false, Ordering::SeqCst);
    }

    pub fn is_clicking(&self) -> bool {
        self.is_clicking.load(Ordering::SeqCst)
    }

    pub fn get_click_count(&self) -> u64 {
        self.click_count.load(Ordering::SeqCst)
    }

    pub fn reset_click_count(&self) {
        self.click_count.store(0, Ordering::SeqCst);
    }

    pub fn get_interval(&self) -> u64 {
        self.config.click_interval_ms
    }

    pub fn set_interval(&mut self, interval: u64) {
        self.config.click_interval_ms = interval;
    }

    pub fn get_mouse_button(&self) -> SerializableMouseButton {
        self.config.mouse_button
    }

    pub fn set_mouse_button(&mut self, button: SerializableMouseButton) {
        self.config.mouse_button = button;
    }

    #[allow(dead_code)]
    pub fn is_random_delay(&self) -> bool {
        self.config.random_delay_enabled
    }

    #[allow(dead_code)]
    pub fn set_random_delay(&mut self, enabled: bool) {
        self.config.random_delay_enabled = enabled;
    }

    pub fn get_random_delay_range(&self) -> (u64, u64) {
        (
            self.config.random_delay_min_ms,
            self.config.random_delay_max_ms,
        )
    }

    pub fn set_random_delay_range(&mut self, min: u64, max: u64) {
        self.config.random_delay_min_ms = min;
        self.config.random_delay_max_ms = max;
    }

    pub fn get_config(&self) -> ClickerConfig {
        self.config.clone()
    }

    pub fn set_config(&mut self, config: ClickerConfig) {
        self.config = config;
    }
}
