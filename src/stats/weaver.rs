use std::{
    sync::{mpsc, Arc, Mutex, Condvar},
    thread,
};

pub const max_threads: u8 = 4;

pub struct Sigourney {
    active_threads: Arc<Mutex<u8>>,
    comm: Arc<Condvar>,
}

impl Sigourney {
    pub fn new () -> Sigourney {
        Sigourney {
            active_threads: Arc::new(Mutex::new(0)),
            comm: Arc::new(Condvar::new()),
        }
    }

    #[allow(dead_code)]
    pub fn thread_count(&self) -> u8 {
        let thread_count = self.active_threads.lock().unwrap();
        *thread_count
    }

    pub fn run_fn<F, T>(&mut self, f: F) -> mpsc::Receiver<T> where 
        F: Send + FnOnce() -> T,
        T: Send,
    {
        let (tx, rx) = mpsc::channel::<T>();
        let active_threads_clone = Arc::clone(&self.active_threads);
        let comm_clone = Arc::clone(&self.comm);
        let fp = move || {
            let mut thread_count = active_threads_clone.lock().unwrap();
            loop {
                if *thread_count < max_threads {
                    break;
                }
                thread_count = comm_clone.wait(thread_count).unwrap();
            }        
            *thread_count += 1;
            drop(thread_count);

            let val = f();
            tx.send(val).unwrap();

            let mut thread_count = active_threads_clone.lock().unwrap();
            *thread_count -= 1;
            drop(thread_count);
            comm_clone.notify_one();
        };

        thread::scope(|s| {
            s.spawn(|| {
                fp();
            });
        });

        rx
    }
}
