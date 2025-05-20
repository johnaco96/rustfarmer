#![allow(dead_code)]
use chrono::{DateTime, Duration, Local};

/// Preconfigured patch locations (1 through 8)
pub const PATCH_NAMES: [&str; 8] = [
    "Falador",
    "Port Phasmatys",
    "Catherby",
    "Ardougne",
    "Hosidius",
    "Trollheim",
    "Weiss",
    "Civitas illa Fortis",
];

#[derive(Debug)]
pub struct HerbTimer {
    pub patch_id: u8,           // 1 through 8
    pub patch_name: String,     // e.g., "Falador"
    pub herb_name: String,      // default: "Herb"
    pub planted_at: DateTime<Local>,
    pub paused: bool,
    pub elapsed_paused: Duration,
    pub last_pause_time: Option<DateTime<Local>>,
}

impl HerbTimer {
    /// Create a new timer for a given patch ID (1-8)
    pub fn new(patch_id: u8) -> Self {
        let patch_name = PATCH_NAMES
            .get((patch_id - 1) as usize)
            .unwrap_or(&"Unknown")
            .to_string();

        let herb_name = "Herb".to_string(); // default herb name

        Self {
            patch_id,
            patch_name,
            herb_name,
            planted_at: Local::now(),
            paused: false,
            elapsed_paused: Duration::zero(),
            last_pause_time: None,
        }
    }


    pub fn pause(&mut self) {
        if !self.paused {
            self.paused = true;
            self.last_pause_time = Some(Local::now());
        }
    }

    pub fn resume(&mut self) {
        if self.paused {
            if let Some(pause_time) = self.last_pause_time {
                self.elapsed_paused = self.elapsed_paused + (Local::now() - pause_time);
            }
            self.paused = false;
            self.last_pause_time = None;
        }
    }

    pub fn reset(&mut self) {
        self.planted_at = Local::now();
        self.elapsed_paused = Duration::zero();
        self.last_pause_time = None;
        self.paused = false;
    }

    /// Time remaining until herb is ready
    pub fn time_remaining(&self) -> Duration {
        if self.paused {
            Duration::minutes(90) - (self.last_pause_time.unwrap() - self.planted_at - self.elapsed_paused)
        } else {
            Duration::minutes(90) - (Local::now() - self.planted_at - self.elapsed_paused)
        }
    }

    /// True if herb is ready to harvest
    pub fn is_ready(&self) -> bool {
        self.time_remaining() <= Duration::zero()
    }

    /// Remaining time as "MM:SS"
    pub fn formatted_remaining(&self) -> String {
        let rem = self.time_remaining();
        let millis = rem.num_milliseconds().max(0);
        let mins = millis / 60_000;
        let secs = (millis / 1000) % 60;
        let subsec = millis % 1000;
        format!("{:02}:{:02}.{:03}", mins, secs, subsec)
    }

    pub fn progress(&self) -> f32 {
        let elapsed = if self.paused {
            self.last_pause_time.unwrap() - self.planted_at - self.elapsed_paused
        } else {
            Local::now() - self.planted_at - self.elapsed_paused
        };
        (elapsed.num_seconds() as f32 / (90.0 * 60.0)).clamp(0.0, 1.0)
    }

}
