use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: String,
    pub num_threads: usize,
    pub default_algorithm: String,
    pub default_scoring: String,
    pub default_num_poses: usize,
    pub default_max_iterations: usize,
    pub output_dir: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            num_threads: 8,
            default_algorithm: "genetic".to_string(),
            default_scoring: "vina".to_string(),
            default_num_poses: 10,
            default_max_iterations: 25000,
            output_dir: ".".to_string(),
        }
    }
}
