use crate::env;
use serde::Serialize;
use serde_json::Value;
use tauri::{AppHandle, Emitter};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub enum DownloadSteps {
    // The download is proceeding setup
    Starting,
    // The download is in progress
    Downloading,
    // The download is extracting the downloaded zip file
    Extracting,
    // The download is cleaning up the extracted files
    Cleaning,
    // The download is complete
    Complete,
}

#[derive(Debug, Clone)]
pub struct GameDownload {
    // The id of the game to download
    pub game_id: u8,
    // The total size of the file to download
    pub file_size: u64,
    // The amount of data downloaded so far
    pub downloaded: u64,
    // The speed of the download in b/s
    pub time_start: Option<std::time::Instant>,
    // The steps of the download
    pub steps: DownloadSteps,
    // The event to broadcast when the download progress changes
    pub event_name: String,

    // ref to the app handle
    app_handle: AppHandle,
}

impl GameDownload {
    pub fn new(game_id: u8, app_handle: AppHandle) -> Self {
        Self {
            game_id,
            file_size: 0,
            downloaded: 0,
            steps: DownloadSteps::Starting,
            event_name: format!("{}_{}", env::EVENT_DOWNLOAD_PROGRESS, game_id),
            app_handle,
            time_start: None,
        }
    }

    pub fn set_file_size(&mut self, file_size: u64) {
        self.file_size = file_size;
    }

    pub fn set_steps(&mut self, steps: DownloadSteps) {
        self.steps = steps;
    }

    pub fn set_start_time(&mut self, time_start: std::time::Instant) {
        self.time_start = Some(time_start);
    }

    pub fn update(&mut self, downloaded: u64, step: Option<DownloadSteps>) {
        self.downloaded = downloaded;
        if let Some(step) = step {
            self.steps = step;
        }
    }

    pub fn advertise(&self) {
        // broadcast the download progress
        self.app_handle
            .emit(&self.event_name, self.get_state())
            .expect("Failed to broadcast the download progress");
    }

    pub fn get_state(&self) -> Value {
        serde_json::json!({
            "game_id": self.game_id,
            "file_size": self.file_size,
            "downloaded": self.downloaded,
            // Calculate the percentage of the download rounded to 2 decimal places
            "percentage": format!("{:.2}%", self.get_percentage()),
            // Format the speed to be in MB/s
            "speed": format!("{:.2} MB/s", self.get_speed_mb()),
            // Calculate the remaining time of the download rounded to 2 decimal places
            "remaining_time": self.get_formated_remaining_time(),
            "steps": self.steps,
        })
    }

    fn get_percentage(&self) -> f64 {
        (self.downloaded as f64 / self.file_size as f64) * 100.0
    }

    fn get_speed_mb(&self) -> f64 {
        if self.time_start.is_none() {
            return 0.0;
        }

        let elapsed = self.time_start.unwrap().elapsed().as_secs_f64();
        if elapsed == 0.0 {
            return 0.0;
        }
        (self.downloaded as f64 / 1024.0 / 1024.0) / elapsed
    }

    fn get_speed_b(&self) -> f64 {
        if self.time_start.is_none() {
            return 0.0;
        }

        let elapsed = self.time_start.unwrap().elapsed().as_secs_f64();
        if elapsed == 0.0 {
            return 0.0;
        }
        (self.downloaded as f64) / elapsed
    }

    fn get_remaining_time(&self) -> f64 {
        (self.file_size - self.downloaded) as f64 / self.get_speed_b()
    }

    pub fn get_formated_remaining_time(&self) -> String {
        let seconds = self.get_remaining_time();
        //format the remaining time to be in hour:minutes or just minutes
        if seconds > 3600.0 {
            format!(
                "{}h {}",
                (seconds / 3600.0).round() as u64,
                ((seconds % 3600.0) / 60.0).round() as u64
            )
        } else if seconds > 60.0 {
            format!("{}min(s)", (seconds / 60.0) as u64)
        } else {
            format!("{}s", seconds as u64)
        }
    }
}
