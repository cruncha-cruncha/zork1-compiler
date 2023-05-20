use std::{
    sync::{mpsc, Arc, Mutex, Condvar},
    thread,
};

pub struct Sigourney {
    max_active_threads: u8,
    active_threads: Arc<Mutex<u8>>,
    total_threads: Arc<Mutex<usize>>,
    comm: Arc<Condvar>,
}

impl Sigourney {
    pub fn new (max_active_threads: u8) -> Sigourney {
        if max_active_threads <= 0 {
            panic!("max_active_threads must be greater than 0");
        }

        Sigourney {
            max_active_threads,
            active_threads: Arc::new(Mutex::new(0)),
            total_threads: Arc::new(Mutex::new(0)),
            comm: Arc::new(Condvar::new()),
        }
    }

    #[allow(dead_code)]
    pub fn max_active_threads(&self) -> u8 {
        self.max_active_threads
    }

    #[allow(dead_code)]
    pub fn active_threads(&self) -> u8 {
        let thread_count = self.active_threads.lock().unwrap();
        *thread_count
    }

    #[allow(dead_code)]
    pub fn total_threads(&self) -> usize {
        let thread_count = self.total_threads.lock().unwrap();
        *thread_count
    }

    pub fn run_fn<F, T>(&mut self, f: F) -> mpsc::Receiver<T> where 
        F: Send + FnOnce() -> T,
        T: Send,
    {
        // get vars the new thread will use
        let (tx, rx) = mpsc::channel::<T>();
        let active_threads_clone = Arc::clone(&self.active_threads);
        let total_threads_clone = Arc::clone(&self.total_threads);
        let comm_clone = Arc::clone(&self.comm);
        let max_threads = self.max_active_threads;

        let fp = move || {
            // increment total_threads
            let mut thread_count = total_threads_clone.lock().unwrap();
            *thread_count += 1;
            drop(thread_count);

            // wait for our turn (can't have more than max_active_threads)
            let mut thread_count = active_threads_clone.lock().unwrap();
            loop {
                if *thread_count < max_threads {
                    break;
                }
                thread_count = comm_clone.wait(thread_count).unwrap();
            }        
            *thread_count += 1;
            drop(thread_count);

            // run
            let val = f();
            tx.send(val).unwrap();

            // we're no longer active
            let mut thread_count = active_threads_clone.lock().unwrap();
            *thread_count -= 1;
            drop(thread_count);

            // decrement total_threads
            let mut thread_count = total_threads_clone.lock().unwrap();
            *thread_count -= 1;
            drop(thread_count);

            // let someone else go
            comm_clone.notify_one();
        };

        // go
        thread::scope(|s| {
            s.spawn(|| {
                fp();
            });
        });

        rx
    }
}
