use crate::config;
use chrono::{Local, Timelike};
use tauri::AppHandle;
use tokio::time::{self, Duration};

pub struct Scheduler {
    app_handle: AppHandle,
    last_run_date: String,
}

impl Scheduler {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            last_run_date: String::new(),
        }
    }

    pub async fn start(&mut self) {
        log::info!("Wallery scheduler started");

        // Run immediately on startup (first wallpaper)
        self.try_update_wallpaper().await;

        // Then check every 60 seconds
        let mut interval = time::interval(Duration::from_secs(60));

        loop {
            interval.tick().await;
            self.try_update_wallpaper().await;
        }
    }

    async fn try_update_wallpaper(&self) {
        let config = config::WalleryConfig::load().unwrap_or_default();

        if !config.schedule.enabled {
            return;
        }

        let now = Local::now();
        let today = now.format("%Y-%m-%d").to_string();

        // Get configured minute-of-day
        let schedule_minutes = config.schedule.hour * 60 + config.schedule.minute;
        let current_minutes = (now.hour() as u32) * 60 + now.minute() as u32;

        let should_update = match config.schedule.frequency.as_str() {
            "daily" => {
                // Run at specific time, only once per day
                current_minutes >= schedule_minutes
                    && current_minutes < schedule_minutes + 5
                    && self.last_run_date != today
            }
            "12h" => {
                let elapsed_minutes = if current_minutes >= schedule_minutes {
                    current_minutes - schedule_minutes
                } else {
                    current_minutes + (1440 - schedule_minutes)
                };
                elapsed_minutes % (12 * 60) < 5
            }
            "6h" => {
                let elapsed_minutes = if current_minutes >= schedule_minutes {
                    current_minutes - schedule_minutes
                } else {
                    current_minutes + (1440 - schedule_minutes)
                };
                elapsed_minutes % (6 * 60) < 5
            }
            "1h" => {
                current_minutes % 60 < 5
            }
            _ => false,
        };

        if should_update {
            log::info!("Scheduler: time to update wallpaper");
            if let Err(e) = crate::commands::wallpaper_cmds::do_next_wallpaper(&self.app_handle).await
            {
                log::error!("Scheduled wallpaper update failed: {}", e);
            }

            // Update run state
            let mut new_config = config;
            new_config.last_update = today.clone();
            let _ = new_config.save();
        }
    }
}
