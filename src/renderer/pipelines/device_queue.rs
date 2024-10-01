use std::sync::Arc;

use wgpu::{Device, Queue};

#[derive(Debug, Clone)]
pub struct DeviceQueue {
    device: Arc<Device>,
    queue: Arc<Queue>,
}

impl DeviceQueue {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> Self {
        Self { device, queue }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}
