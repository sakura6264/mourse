use crate::clicker::Clicker;
use crate::mouse_button::SerializableMouseButton;
use crate::mouse_mover::MouseMover;
use device_query::{DeviceQuery, DeviceState, Keycode};
use eframe::egui;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

#[derive(Debug)]
pub struct MourseApp {
    clicker: Clicker,
    mouse_mover: MouseMover,
    device_state: DeviceState,
    last_key_press: std::time::Instant,
    config_path: PathBuf,
}

impl MourseApp {
    fn get_config_path() -> PathBuf {
        env::current_exe()
            .expect("Failed to get executable path")
            .parent()
            .expect("Failed to get executable directory")
            .join("config.ron")
    }

    fn save_config(&self) {
        let config = (self.clicker.get_config(), self.mouse_mover.get_config());
        if let Ok(config_str) =
            ron::ser::to_string_pretty(&config, ron::ser::PrettyConfig::default())
        {
            if let Err(e) = fs::write(&self.config_path, config_str) {
                eprintln!("Failed to save config: {}", e);
            }
        }
    }

    fn load_config(&mut self) {
        if let Ok(config_str) = fs::read_to_string(&self.config_path) {
            if let Ok((clicker_config, mover_config)) = ron::from_str(&config_str) {
                self.clicker.set_config(clicker_config);
                self.mouse_mover.set_config(mover_config);
            }
        }
    }

    fn open_config_file(&self) {
        if self.config_path.exists() {
            #[cfg(target_os = "windows")]
            {
                Command::new("notepad").arg(&self.config_path).spawn().ok();
            }
            #[cfg(target_os = "macos")]
            {
                Command::new("open").arg(&self.config_path).spawn().ok();
            }
            #[cfg(target_os = "linux")]
            {
                Command::new("xdg-open").arg(&self.config_path).spawn().ok();
            }
        } else {
            self.save_config();
            self.open_config_file();
        }
    }
}

impl Default for MourseApp {
    fn default() -> Self {
        let mut app = Self {
            clicker: Clicker::default(),
            mouse_mover: MouseMover::default(),
            device_state: DeviceState::new(),
            last_key_press: std::time::Instant::now(),
            config_path: Self::get_config_path(),
        };
        app.load_config();
        app
    }
}

impl eframe::App for MourseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for hotkeys with debouncing
        let keys: Vec<Keycode> = self.device_state.get_keys();
        let now = std::time::Instant::now();

        if keys.contains(&Keycode::F6) {
            if now.duration_since(self.last_key_press) > Duration::from_millis(200) {
                if self.clicker.is_clicking() {
                    self.clicker.stop_clicking();
                } else {
                    self.clicker.start_clicking();
                }
                self.last_key_press = now;
            }
        }

        if keys.contains(&Keycode::F7) {
            if now.duration_since(self.last_key_press) > Duration::from_millis(200) {
                if self.mouse_mover.is_moving() {
                    self.mouse_mover.stop_moving();
                } else {
                    self.mouse_mover.start_moving();
                }
                self.last_key_press = now;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                // Title with some spacing
                ui.add_space(5.0);
                ui.heading("Mourse");
                ui.add_space(5.0);

                // Add config file button at the top level
                if ui.button("Open Config File").clicked() {
                    self.open_config_file();
                }
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        // Auto Clicker Settings
                        ui.group(|ui| {
                            ui.set_width(250.0);
                            ui.heading("Auto Clicker");

                            ui.horizontal(|ui| {
                                ui.label("Clicks:");
                                ui.label(format!("{}", self.clicker.get_click_count()));
                                if ui.small_button("Reset").clicked() {
                                    self.clicker.reset_click_count();
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Interval (ms):");
                                let mut interval = self.clicker.get_interval();
                                let slider = egui::Slider::new(&mut interval, 10..=1000);
                                if ui.add(slider).changed() {
                                    self.clicker.set_interval(interval);
                                    self.save_config();
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Button:");
                                let mut button = self.clicker.get_mouse_button();
                                egui::ComboBox::from_label("")
                                    .selected_text(format!("{:?}", button))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut button,
                                            SerializableMouseButton::Left,
                                            "Left",
                                        );
                                        ui.selectable_value(
                                            &mut button,
                                            SerializableMouseButton::Right,
                                            "Right",
                                        );
                                        ui.selectable_value(
                                            &mut button,
                                            SerializableMouseButton::Middle,
                                            "Middle",
                                        );
                                    });
                                if button != self.clicker.get_mouse_button() {
                                    self.clicker.set_mouse_button(button);
                                    self.save_config();
                                }
                            });

                            if ui
                                .checkbox(
                                    &mut self.clicker.config.random_delay_enabled,
                                    "Random Interval",
                                )
                                .changed()
                            {
                                self.save_config();
                            }

                            if self.clicker.config.random_delay_enabled {
                                ui.horizontal(|ui| {
                                    ui.label("Extra Delay Range:");
                                    let (mut min, mut max) = self.clicker.get_random_delay_range();
                                    let mut changed = false;
                                    changed |= ui
                                        .add(
                                            egui::DragValue::new(&mut min)
                                                .speed(1.0)
                                                .range(0..=1000)
                                                .suffix(" ms"),
                                        )
                                        .changed();
                                    ui.label("to");
                                    changed |= ui
                                        .add(
                                            egui::DragValue::new(&mut max)
                                                .speed(1.0)
                                                .range(min..=1000)
                                                .suffix(" ms"),
                                        )
                                        .changed();
                                    if changed {
                                        self.clicker.set_random_delay_range(min, max);
                                        self.save_config();
                                    }
                                });
                            }

                            let clicking_text = if self.clicker.is_clicking() {
                                "Stop Clicking (F6)"
                            } else {
                                "Start Clicking (F6)"
                            };
                            if ui.button(clicking_text).clicked() {
                                if self.clicker.is_clicking() {
                                    self.clicker.stop_clicking();
                                } else {
                                    self.clicker.start_clicking();
                                }
                            }
                        });

                        ui.add_space(5.0);

                        // Mouse Mover Settings
                        ui.group(|ui| {
                            ui.set_width(250.0);
                            ui.heading("Random Mouse Mover");

                            ui.horizontal(|ui| {
                                ui.label("Moves:");
                                ui.label(format!("{}", self.mouse_mover.get_move_count()));
                                if ui.small_button("Reset").clicked() {
                                    self.mouse_mover.reset_move_count();
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Move Interval (ms):");
                                let mut interval = self.mouse_mover.get_interval();
                                let slider = egui::Slider::new(&mut interval, 10..=500);
                                if ui.add(slider).changed() {
                                    self.mouse_mover.set_interval(interval);
                                    self.save_config();
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Max Distance:");
                                let mut max_distance = self.mouse_mover.get_max_distance();
                                let slider = egui::Slider::new(&mut max_distance, 10..=500);
                                if ui.add(slider).changed() {
                                    self.mouse_mover.set_max_distance(max_distance);
                                    self.save_config();
                                }
                            });

                            let mut random_delay = self.mouse_mover.is_random_delay();
                            if ui.checkbox(&mut random_delay, "Random Interval").changed() {
                                self.mouse_mover.set_random_delay(random_delay);
                                self.save_config();
                            }

                            if random_delay {
                                ui.horizontal(|ui| {
                                    ui.label("Extra Delay Range:");
                                    let (mut min, mut max) =
                                        self.mouse_mover.get_random_delay_range();
                                    let mut changed = false;
                                    changed |= ui
                                        .add(
                                            egui::DragValue::new(&mut min)
                                                .speed(1.0)
                                                .range(0..=500)
                                                .suffix(" ms"),
                                        )
                                        .changed();
                                    ui.label("to");
                                    changed |= ui
                                        .add(
                                            egui::DragValue::new(&mut max)
                                                .speed(1.0)
                                                .range(min..=500)
                                                .suffix(" ms"),
                                        )
                                        .changed();
                                    if changed {
                                        self.mouse_mover.set_random_delay_range(min, max);
                                        self.save_config();
                                    }
                                });
                            }

                            let moving_text = if self.mouse_mover.is_moving() {
                                "Stop Moving (F7)"
                            } else {
                                "Start Moving (F7)"
                            };
                            if ui.button(moving_text).clicked() {
                                if self.mouse_mover.is_moving() {
                                    self.mouse_mover.stop_moving();
                                } else {
                                    self.mouse_mover.start_moving();
                                }
                            }
                        });
                    });
                });
            });
        });

        // Request a repaint to keep checking for key presses
        ctx.request_repaint();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_config();
    }
}
