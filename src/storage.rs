use crate::core::DockingResult;
use std::sync::Mutex;

pub struct PoseStorage {
    pub best_pose: Option<DockingResult>,
}

impl PoseStorage {
    pub fn new() -> Self {
        Self { best_pose: None }
    }
    
    pub fn store_best(&mut self, result: DockingResult) {
        self.best_pose = Some(result);
    }
    
    pub fn get_best(&self) -> Option<&DockingResult> {
        self.best_pose.as_ref()
    }
}

// Global pose storage instance
lazy_static::lazy_static! {
    pub static ref POSE_STORAGE: Mutex<PoseStorage> = Mutex::new(PoseStorage::new());
}
