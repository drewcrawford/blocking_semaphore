/*!
Semaphores which are only "one-high", they can hold values of 0-1.
*/

use std::hash::Hash;
use std::sync::{Arc, Condvar};
use dlog::perfwarn;

#[derive(Debug)]
struct Shared {
    c: Condvar,
    m: std::sync::Mutex<bool>
}

#[derive(Debug,Clone)]
pub struct Semaphore {
    shared: Arc<Shared>
}

impl PartialEq for Semaphore {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.shared, &other.shared)
    }
}

impl Eq for Semaphore {}

impl Hash for Semaphore {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.shared).hash(state);
    }
}

impl Default for Semaphore {
    /**
    The default Semaphore is unsignaled.
*/
    fn default() -> Self {
        Self::new(false)
    }
}


impl Semaphore {
    /**
    Creates a new semaphore, specifying if it is initially signalled.
*/
    pub fn new(initially_signaled: bool) -> Semaphore {
        Semaphore {
            shared: Arc::new(Shared {
                c: Condvar::new(),
                m: std::sync::Mutex::new(initially_signaled)
            })
        }
    }
}

impl Semaphore {
    /**
    Signals (increments) the semaphore.

    It is a programming error to signal a semaphore that is already signalled.  To do this, use [signal_if_needed].
*/
    pub fn signal(&self) {
        {
            dlog::trace_sync!("signal");
            perfwarn!("Semaphore implementation uses mutex", {
                dlog::trace_sync!("waiting for mutex");
                let mut guard = self.shared.m.lock().unwrap();
                dlog::trace_sync!("arrived");
                assert!(!*guard, "Signalling a semaphore that is already signalled");
                *guard = true;
                self.shared.c.notify_one();
            });
        }
    }

    /**
    Signals (increments) the semaphore if it is not already signalled.

    Like [signal], but does nothing if the semaphore is already signaled.
*/
    pub fn signal_if_needed(&self) {
        {
            dlog::trace_sync!("signal_if_needed");
            perfwarn!("Semaphore implementation uses mutex", {
                dlog::trace_sync!("waiting for mutex");
                let mut guard = self.shared.m.lock().unwrap();
                dlog::trace_sync!("arrived");
                *guard = true;
            });

        }
    }

    /**Waits (decrements) the semaphore.
    */
    pub fn wait(&self) {
        dlog::trace_sync!("wait");
        perfwarn!("Semaphore implementation uses mutex", {
            dlog::trace_sync!("waiting for mutex");
            let mtx = self.shared.m.lock().unwrap();
            dlog::trace_sync!("arrived");
            let mut g = self.shared.c.wait_while(mtx, |guard| !*guard).unwrap();
            *g = false;
        });
        dlog::trace_sync!("finished waiting");


    }



}

#[cfg(test)] mod test {
    #[test] fn test_semaphore() {
        dlog::context::Context::reset();
        let s = super::Semaphore::new(false);
        s.signal();
        s.wait();
    }
}