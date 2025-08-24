use enigo::{Coordinate, Enigo, Mouse, Settings};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MouseMoverConfig {
    pub move_interval_ms: u64,
    pub max_distance: i32,
    pub random_delay_enabled: bool,
    pub random_delay_min_ms: u64,
    pub random_delay_max_ms: u64,
}

impl Default for MouseMoverConfig {
    fn default() -> Self {
        Self {
            move_interval_ms: 100,
            max_distance: 100,
            random_delay_enabled: false,
            random_delay_min_ms: 0,
            random_delay_max_ms: 200,
        }
    }
}

#[derive(Debug)]
pub struct MouseMover {
    pub config: MouseMoverConfig,
    is_moving: Arc<AtomicBool>,
    move_count: Arc<AtomicU64>,
}

impl Default for MouseMover {
    fn default() -> Self {
        Self::new(MouseMoverConfig::default())
    }
}

impl MouseMover {
    pub fn new(config: MouseMoverConfig) -> Self {
        Self {
            config,
            is_moving: Arc::new(AtomicBool::new(false)),
            move_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn start_moving(&mut self) {
        if !self.is_moving.load(Ordering::SeqCst) {
            self.is_moving.store(true, Ordering::SeqCst);
            let is_moving = Arc::clone(&self.is_moving);
            let move_count = Arc::clone(&self.move_count);
            let config = self.config.clone();

            thread::spawn(move || {
                let settings = Settings::default();
                let mut enigo = Enigo::new(&settings).expect("Failed to create Enigo instance");
                let mut rng = rand::rng();

                while is_moving.load(Ordering::SeqCst) {
                    let dx = rng.random_range(-config.max_distance..=config.max_distance);
                    let dy = rng.random_range(-config.max_distance..=config.max_distance);
                    let _ = enigo.move_mouse(dx, dy, Coordinate::Rel);
                    move_count.fetch_add(1, Ordering::SeqCst);

                    let delay = if config.random_delay_enabled {
                        config.move_interval_ms
                            + rng.random_range(
                                config.random_delay_min_ms..=config.random_delay_max_ms,
                            )
                    } else {
                        config.move_interval_ms
                    };

                    thread::sleep(Duration::from_millis(delay));
                }
            });
        }
    }

    pub fn stop_moving(&mut self) {
        self.is_moving.store(false, Ordering::SeqCst);
    }

    pub fn is_moving(&self) -> bool {
        self.is_moving.load(Ordering::SeqCst)
    }

    pub fn get_interval(&self) -> u64 {
        self.config.move_interval_ms
    }

    pub fn set_interval(&mut self, interval: u64) {
        self.config.move_interval_ms = interval;
    }

    pub fn get_max_distance(&self) -> i32 {
        self.config.max_distance
    }

    pub fn set_max_distance(&mut self, distance: i32) {
        self.config.max_distance = distance;
    }

    pub fn is_random_delay(&self) -> bool {
        self.config.random_delay_enabled
    }

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

    pub fn get_move_count(&self) -> u64 {
        self.move_count.load(Ordering::SeqCst)
    }

    pub fn reset_move_count(&self) {
        self.move_count.store(0, Ordering::SeqCst);
    }

    pub fn get_config(&self) -> MouseMoverConfig {
        self.config.clone()
    }

    pub fn set_config(&mut self, config: MouseMoverConfig) {
        // Preserve the runtime state when loading config
        self.config = config;
    }
}
