use eframe::{egui, App, CreationContext};
use std::collections::HashMap;
use std::time::Duration;

mod timer;
use timer::{HerbTimer, PATCH_NAMES};

pub struct HerbApp {
    timers: HashMap<u8, HerbTimer>,
}

impl HerbApp {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {
            timers: HashMap::new(),
        }
    }
}

impl App for HerbApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🌿 Herb Patches");
            ui.separator();

            for id in 1..=8 {
                let patch_name = PATCH_NAMES[(id - 1) as usize];
                ui.horizontal(|ui| {
                    ui.set_height(40.0);

                    match self.timers.get_mut(&id) {
                        Some(timer) => {
                            let remaining = timer.time_remaining();
                            let total = Duration::from_secs(90 * 60);
                            let progress = (1.0 - remaining.as_seconds_f32() / total.as_secs_f32()).clamp(0.0, 1.0);
                            let time_str = timer.formatted_remaining();

                            ui.label(format!("{id}. {patch_name} — {time_str}"));
                            ui.add(
                                egui::ProgressBar::new(progress)
                                    .desired_width(150.0)
                                    .show_percentage(),
                            );

                            // Render pause/resume and reset buttons
                            let actions: [(&str, fn(&mut HerbTimer)); 2] = if timer.paused {
                                [("▶ Resume", HerbTimer::resume), ("Reset", HerbTimer::reset)]
                            } else {
                                [("⏸ Pause", HerbTimer::pause), ("Reset", HerbTimer::reset)]
                            };

                            for (label, action) in actions {
                                if ui.button(label).clicked() {
                                    action(timer);
                                }
                            }
                        }

                        None => {
                            let button = egui::Button::new(format!("Herb at {patch_name}"))
                                .min_size(egui::vec2(220.0, 32.0));
                            if ui.add(button).clicked() {
                                self.timers.insert(id, HerbTimer::new(id));
                            }
                        }
                    }
                });

                ui.add_space(10.0); // Add vertical spacing between rows
            }

            // Force redraw to keep progress bars and timers updated
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Herb Growth Tracker!",
        options,
        Box::new(|cc| Box::new(HerbApp::new(cc))),
    )
}
