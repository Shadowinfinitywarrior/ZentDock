use chrono::{DateTime, Utc};
use crate::core::DockingResult;

#[derive(Debug, Clone)]
pub struct JobStatus {
    pub id: String,
    pub status: String,
    pub progress: f64,
    pub energy: f64,
    pub start_time: DateTime<Utc>,
}

pub struct ActivityMonitor;

impl ActivityMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn start_job(&self, job_id: &str) -> JobStatus {
        JobStatus {
            id: job_id.to_string(),
            status: "Running".to_string(),
            progress: 0.0,
            energy: 0.0,
            start_time: Utc::now(),
        }
    }

    pub fn update(&self, job_id: &str, progress: f64, energy: f64) {
        if progress.trunc() as i64 % 5000 == 0 {
            println!("[MONITOR] Job {} - Iteration {:.0}/25000 ({:.0}%) - Best energy: {:.2} kcal/mol", 
                job_id, progress, (progress / 25000.0) * 100.0, energy);
        }
    }

    pub fn complete(&self, job_id: &str, final_energy: f64) {
        println!("[MONITOR] Job {} - Completed - Final energy: {:.2} kcal/mol", job_id, final_energy);
    }

    pub fn verbose_output(&self, results: &[DockingResult]) {
        println!("\n=== Docking Results ===");
        for result in results {
            println!("  Rank {}: Energy = {:.2} kcal/mol", result.rank, result.energy);
        }
    }
}
