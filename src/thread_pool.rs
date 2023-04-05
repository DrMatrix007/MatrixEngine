use std::{
    marker::PhantomData,
    sync::{
        atomic::{AtomicIsize, Ordering},
        mpsc::{channel, Receiver, SendError, Sender},
        Arc, Mutex,
    },
    thread::JoinHandle,
};

pub enum Job<T> {
    Work(Box<dyn FnOnce() -> T + Send>),
    Close,
}
impl<T> Job<T> {
    fn try_get_func(self) -> Option<Box<dyn FnOnce() -> T + Send>> {
        match self {
            Job::Work(f) => Some(f),
            Job::Close => None,
        }
    }
}

struct Worker<T: Send> {
    _handle: JoinHandle<()>,
    marker: PhantomData<T>,
}

impl<T: Send + 'static> Worker<T> {
    fn new(jobs: Arc<Mutex<Receiver<Job<T>>>>, done: Sender<T>) -> Self {
        Self {
            _handle: std::thread::spawn(|| Worker::run_thread(jobs, done)),
            marker: PhantomData,
        }
    }
    fn run_thread(jobs: Arc<Mutex<Receiver<Job<T>>>>, done: Sender<T>) {
        loop {
            let job = jobs.lock().expect("the value should be accessible").recv();
            match job {
                Ok(job) => match job {
                    Job::Work(job) => {
                        let ans = job();
                        match done.send(ans) {
                            Err(_) => return,
                            Ok(_) => {}
                        }
                    }
                    Job::Close => {
                        return;
                    }
                },
                Err(_) => {
                    return;
                }
            }
        }
    }
}

pub struct ThreadPool<T: Send> {
    _threads: Vec<Worker<T>>,
    job_sender: Sender<Job<T>>,
    data_receiver: Receiver<T>,
    job_counter: Arc<AtomicIsize>,
}

impl<T: Send + 'static> ThreadPool<T> {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);
        let (job_sender, job_receiver) = channel();
        let (data_sender, data_receiver) = channel();
        let job_receive = Arc::new(Mutex::new(job_receiver));
        for _ in 0..size {
            threads.push(Worker::new(job_receive.clone(), data_sender.clone()));
        }

        Self {
            _threads: threads,
            job_sender,
            data_receiver,
            job_counter: Arc::new(0.into()),
        }
    }

    pub fn add<F: FnOnce() -> T + Send + 'static>(
        &self,
        f: F,
    ) -> Result<(), SendError<Box<dyn FnOnce() -> T + Send + 'static>>> {
        match self.job_sender.send(Job::Work(Box::new(f))) {
            Ok(_) => {
                self.job_counter.fetch_add(1, Ordering::Acquire);
                Ok(())
            }
            Err(d) => Err(SendError(
                d.0.try_get_func()
                    .expect("this value should containt a funciton"),
            )),
        }
    }

    pub fn join(self) {
        for _ in 0..self._threads.len() {
            self.job_sender.send(Job::Close).unwrap();
        }
        for t in self._threads {
            t._handle.join().unwrap();
        }
    }
    pub fn try_recv_iter(&self) -> ThreadPoolTryRecvIter<'_, T> {
        ThreadPoolTryRecvIter {
            recv: &self.data_receiver,
            job_counter: self.job_counter.clone(),
        }
    }
    pub fn recv_iter(&self) -> ThreadPoolRecvIter<'_, T> {
        ThreadPoolRecvIter {
            recv: &self.data_receiver,
            job_counter: self.job_counter.clone(),
        }
    }

    pub fn wait_for_all(&self) {
        for _ in self.recv_iter() {}
    }
    pub fn sender(&self) -> ThreadPoolSender<T> {
        ThreadPoolSender {
            sender: self.job_sender.clone(),
            counter: self.job_counter.clone(),
        }
    }
}
pub struct ThreadPoolTryRecvIter<'a, T> {
    recv: &'a Receiver<T>,
    job_counter: Arc<AtomicIsize>,
}

impl<'a, T> Iterator for ThreadPoolTryRecvIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.recv.try_recv().ok() {
            Some(data) => {
                self.job_counter.fetch_sub(1, Ordering::SeqCst);
                Some(data)
            }
            None => None,
        }
    }
}
pub struct ThreadPoolRecvIter<'a, T> {
    recv: &'a Receiver<T>,
    job_counter: Arc<AtomicIsize>,
}

impl<'a, T> Iterator for ThreadPoolRecvIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.job_counter.fetch_sub(1, Ordering::SeqCst);
        if c == 0 {
            assert!(self.job_counter.fetch_add(1, Ordering::SeqCst) == -1);
            return None;
        }
        match self.recv.recv().ok() {
            Some(data) => Some(data),
            None => None,
        }
    }
}

pub struct ThreadPoolSender<T> {
    pub(self) sender: Sender<Job<T>>,
    pub(self) counter: Arc<AtomicIsize>,
}

impl<T> ThreadPoolSender<T> {
    pub fn send(&self, job: Job<T>) -> Result<(), SendError<Job<T>>> {
        self.sender.send(job).map(|x| {
            self.counter.fetch_add(1, Ordering::Acquire);
            x
        })
    }
}

mod tests {

    #[test]
    pub fn thread_pool_test() {
        use crate::thread_pool::ThreadPool;
        use std::collections::HashSet;

        let pool = ThreadPool::new(10);

        let mut data = (0..200).collect::<HashSet<_>>();

        for i in data.iter() {
            let i = i * i - i;
            let _ = i128::pow(i, 4);
        }
        for i in data.iter().copied() {
            pool.add(move || i).unwrap();
        }
        for i in pool.recv_iter() {
            data.remove(&i);
        }
        println!("{:?}", data);
    }
}
