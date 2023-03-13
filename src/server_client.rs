use std::sync::mpsc::{channel, Receiver, RecvError, SendError, Sender};

pub struct Response<T> {
    data: T,
}

impl<T> Response<T> {
    pub fn unpack(self) -> T {
        self.data
    }
}
pub struct Request<T, M> {
    data: T,
    sender: Sender<Response<M>>,
}
impl<T, M> Request<T, M> {
    pub fn new(data: T, sender: Sender<Response<M>>) -> Self {
        Self { data, sender }
    }

    pub fn unpack(self) -> (T, RequestSender<M>) {
        (self.data, RequestSender::<M>(self.sender))
    }
}

pub struct RequestSender<M>(Sender<Response<M>>);

impl<M> RequestSender<M> {
    pub fn send(self, data: M) -> Result<(), SendError<Response<M>>> {
        self.0.send(Response { data })
    }
}

pub struct ServerBuilder<T, M> {
    rec: Receiver<Request<T, M>>,
    sender: Sender<Request<T, M>>,
}

impl<T, M> Default for ServerBuilder<T, M> {
    fn default() -> Self {
        let (s, r) = channel();
        Self { rec: r, sender: s }
    }
}

impl<T, M> ServerBuilder<T, M> {
    pub fn sender(&self) -> Sender<Request<T, M>> {
        self.sender.clone()
    }
    pub fn build(self) -> Server<T, M> {
        Server { rec: self.rec }
    }
}
pub struct Server<T, M> {
    rec: Receiver<Request<T, M>>,
}

impl<T, M> Server<T, M> {
    pub fn recv(&self) -> Result<Request<T, M>, RecvError> {
        self.rec.recv()
    }
}

#[derive(Debug)]
pub struct Client<Req, Res> {
    server: Sender<Request<Req, Res>>,
    receiver: Receiver<Response<Res>>,
    sender: Sender<Response<Res>>,
}

impl<T, M> Client<T, M> {
    pub fn new(server: Sender<Request<T, M>>) -> Self {
        let (a, b) = channel();
        Self {
            server,
            receiver: b,
            sender: a,
        }
    }

    pub fn sender(&self) -> Sender<Response<M>> {
        self.sender.clone()
    }
    pub fn server_sender(&self) -> Sender<Request<T, M>> {
        self.server.clone()
    }
    pub fn send(&self, data: T) -> Result<(), SendError<Request<T, M>>> {
        self.server.send(Request {
            data,
            sender: self.sender(),
        })
    }
    pub fn recv(&self) -> Result<Response<M>, RecvError> {
        self.receiver.recv()
    }
}
