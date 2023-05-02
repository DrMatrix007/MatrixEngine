use std::{any::TypeId, sync::mpsc::{Sender, Receiver, channel}};

pub enum MatrixEvent {
    CreatedResource(TypeId),
    RemovedResource(TypeId),
}


#[derive(Clone)]
pub struct MatrixEventSender {
    sender: Sender<MatrixEvent>,
}

impl MatrixEventSender {
    pub fn send(&self, e: MatrixEvent) -> Result<(), std::sync::mpsc::SendError<MatrixEvent>> {
        self.sender.send(e)
    }
}

impl MatrixEventSender {
    fn new(sender: Sender<MatrixEvent>) -> Self {
        Self { sender }
    }
}
pub struct MatrixEventReceiver {
    recv: Receiver<MatrixEvent>,
    
}

impl MatrixEventReceiver {
    fn new(recv: Receiver<MatrixEvent>) -> Self {
        Self { recv }
    }
    pub fn iter_current(&self) -> std::sync::mpsc::TryIter<MatrixEvent> {
        self.recv.try_iter()
    }
}

pub(crate) fn channel_matrix_event() -> (MatrixEventSender, MatrixEventReceiver) {
    let (a, b) = channel();
    (MatrixEventSender::new(a), MatrixEventReceiver::new(b))
}
