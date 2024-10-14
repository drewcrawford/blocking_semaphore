// SPDX-License-Identifier: MIT OR Apache-2.0

/*!
A binary semaphore; which are only "one-high", they can hold values of 0-1.
*/

use std::hash::Hash;
use std::sync::{Arc, Condvar};
use logwise::perfwarn;

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

    It is a programming error to signal a semaphore that is already signalled.  To do this, use [Self::signal_if_needed].
*/
    pub fn signal(&self) {
        {

            logwise::trace_sync!("signal");
            perfwarn!("Semaphore implementation uses mutex", {
                logwise::trace_sync!("waiting for mutex");
                let mut guard = self.shared.m.lock().unwrap();
                logwise::trace_sync!("arrived");
                assert!(!*guard, "Signalling a semaphore that is already signalled");
                *guard = true;
                self.shared.c.notify_one();
            });
        }
    }

    /**
    Signals (increments) the semaphore if it is not already signalled.

    Like [Self::signal], but does nothing if the semaphore is already signaled.
*/
    pub fn signal_if_needed(&self) {
        {
            logwise::trace_sync!("signal_if_needed");
            perfwarn!("Semaphore implementation uses mutex", {
                logwise::trace_sync!("waiting for mutex");
                let mut guard = self.shared.m.lock().unwrap();
                logwise::trace_sync!("arrived");
                *guard = true;
                self.shared.c.notify_one();
            });

        }
    }

    /**Waits (decrements) the semaphore.
    */
    pub fn wait(&self) {
        logwise::trace_sync!("wait");
        perfwarn!("Semaphore implementation uses mutex", {
            logwise::trace_sync!("waiting for mutex");
            let mtx = self.shared.m.lock().unwrap();
            logwise::trace_sync!("arrived.  Wait_while...");
            let mut g = self.shared.c.wait_while(mtx, |guard| {
                logwise::trace_sync!("...wait_while: {guard}", guard=*guard);
                !*guard
                }
            ).unwrap();
            logwise::trace_sync!("...finished wait-while.");

            *g = false;
        });
        logwise::trace_sync!("finished waiting");


    }



}

#[cfg(test)] mod test {
    #[test] fn test_semaphore() {
        logwise::context::Context::reset("test_semaphore");
        let s = super::Semaphore::new(false);
        s.signal();
        s.wait();
    }
}