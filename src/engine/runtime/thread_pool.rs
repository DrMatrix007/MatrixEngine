use std::{
    marker::PhantomData,
    sync::{
        atomic::{AtomicIsize, Ordering},
        mpsc::{channel, Receiver, SendError, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
};
use tokio::sync::Mutex;

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

#[derive(Debug, Clone, Copy)]
pub enum WorkerError {
    Panic,
}

pub type JobResult<T> = Result<T, WorkerError>;

struct Worker<T: Send> {
    _handle: JoinHandle<()>,
    marker: PhantomData<T>,
}

#[derive(Debug, Clone)]
struct PosionPill<'a, T>(&'a Sender<JobResult<T>>);

impl<'a, T> PosionPill<'a, T> {
    pub fn new(sender: &'a Sender<JobResult<T>>) -> Self {
        Self(sender)
    }
}

impl<'a, T> Drop for PosionPill<'a, T> {
    fn drop(&mut self) {
        if thread::panicking() {
            self.0
                .send(Err(WorkerError::Panic))
                .expect("this function should send this data");
        }
    }
}

impl<T: Send + 'static> Worker<T> {
    fn new(
        jobs: Arc<Mutex<Receiver<Job<T>>>>,
        done: Sender<JobResult<T>>,
        proxies: Arc<Mutex<Vec<Box<dyn FnMut(&mut T) + Send>>>>,
    ) -> Self {
        Self {
            _handle: std::thread::spawn(|| Worker::run_thread(jobs, done, proxies)),
            marker: PhantomData,
        }
    }
    fn run_thread(
        jobs: Arc<Mutex<Receiver<Job<T>>>>,
        done: Sender<JobResult<T>>,
        proxies: Arc<Mutex<Vec<Box<dyn FnMut(&mut T) + Send>>>>,
    ) {
        let _pill = PosionPill::new(&done);
        loop {
            let mutex_lock = jobs.blocking_lock();
            let job = mutex_lock.recv();
            drop(mutex_lock);
            match job {
                Ok(job) => match job {
                    Job::Work(job) => {
                        let mut ans = job();

                        let mut proxies = proxies.blocking_lock();

                        for p in proxies.iter_mut() {
                            p(&mut ans);
                        }

                        drop(proxies);

                        if done.send(Ok(ans)).is_err() {
                            return;
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
    threads: Vec<Worker<T>>,
    job_sender: Sender<Job<T>>,
    data_receiver: Receiver<JobResult<T>>,
    job_counter: Arc<AtomicIsize>,
    proxies: Arc<Mutex<Vec<Box<dyn FnMut(&mut T) + Send>>>>,
}

impl<T: Send + 'static> ThreadPool<T> {
    pub fn new(threads_count: usize) -> Self {
        let proxies = Arc::new(Mutex::new(vec![]));
        assert!(threads_count > 0);
        let mut threads = Vec::with_capacity(threads_count);
        let (job_sender, job_receiver) = channel();
        let (data_sender, data_receiver) = channel();
        let job_receive = Arc::new(Mutex::new(job_receiver));
        for _ in 0..threads_count {
            threads.push(Worker::new(
                job_receive.clone(),
                data_sender.clone(),
                proxies.clone(),
            ));
        }

        Self {
            threads,
            job_sender,
            data_receiver,
            proxies,
            job_counter: Arc::new(0.into()),
        }
    }
    pub fn add_proxy(&self,f:impl FnMut(&mut T) + Send+'static) {
        self.proxies.blocking_lock().push(Box::new(f));
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
        for _ in 0..self.threads.len() {
            self.job_sender.send(Job::Close).unwrap();
        }
        for t in self.threads {
            t._handle.join().expect("a thread panicked");
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
    recv: &'a Receiver<JobResult<T>>,
    job_counter: Arc<AtomicIsize>,
}

impl<'a, T> Iterator for ThreadPoolTryRecvIter<'a, T> {
    type Item = JobResult<T>;

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
    recv: &'a Receiver<JobResult<T>>,
    job_counter: Arc<AtomicIsize>,
}

impl<'a, T> Iterator for ThreadPoolRecvIter<'a, T> {
    type Item = JobResult<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.job_counter.fetch_sub(1, Ordering::SeqCst);
        if c == 0 {
            assert!(self.job_counter.fetch_add(1, Ordering::SeqCst) == -1);
            return None;
        }
        self.recv.recv().ok()
    }
}

pub struct ThreadPoolSender<T> {
    pub(self) sender: Sender<Job<T>>,
    pub(self) counter: Arc<AtomicIsize>,
}

impl<T> ThreadPoolSender<T> {
    pub fn send<F>(&self, job: F) -> Result<(), SendError<Job<T>>>
    where
        F: FnOnce() -> T + Send + 'static,
    {
        self.sender.send(Job::Work(Box::new(job))).map(|x| {
            self.counter.fetch_add(1, Ordering::Acquire);
            x
        })
    }
}

mod tests {
    #[test]
    pub fn thread_pool_test() {
        use crate::engine::runtime::thread_pool::ThreadPool;
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
            data.remove(&i.unwrap());
        }
        println!("{:?}", data);
    }

    #[test]
    pub fn test_mutex() {
        use super::ThreadPool;
        use std::sync::Arc;
        use tokio::sync::Mutex;

        let m = Arc::new(Mutex::new(0));
        let mm = m.clone();
        let pool = ThreadPool::new(2);

        pool.add(move || {
            let _ = m.blocking_lock();
        })
        .unwrap();

        let _ = mm.blocking_lock();
    }
}
