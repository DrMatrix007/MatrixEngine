use wgpu::{Device, Queue};

#[derive(Debug, Clone)]
pub struct DeviceQueue {
    device: Device,
    queue: Queue,
}

impl AsRef<Queue> for DeviceQueue {
    fn as_ref(&self) -> &Queue {
        &self.queue
    }
}

impl AsRef<Device> for DeviceQueue {
    fn as_ref(&self) -> &Device {
        &self.device
    }
}

impl DeviceQueue {
    pub fn new(device: Device, queue: Queue) -> Self {
        Self { device, queue }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}
