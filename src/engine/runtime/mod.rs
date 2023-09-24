pub mod thread_pool;

trait Runtime {
    fn add_async(&mut self, s: ());
    fn add_sync(&mut self, s: ());
}
