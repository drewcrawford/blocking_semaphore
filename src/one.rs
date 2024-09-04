/*!
Semaphores which are only "one-high", they can hold values of 0-1.
*/

use std::sync::Condvar;
use dlog::perfwarn;

pub struct Semaphore {
    c: Condvar,
    m: std::sync::Mutex<bool>
}

impl Semaphore {
    fn signal(&self) {
        {
            perfwarn!("Semaphore implementation uses mutex", {
                let mut guard = self.m.lock().unwrap();
                assert!(!*guard, "Signalling a semaphore that is already signalled");
                *guard = true;
                self.c.notify_one();

            });

        }
    }

    fn wait(&self) {
        perfwarn!("Semaphore implementation uses mutex", {
            let mut g = self.c.wait_while(self.m.lock().unwrap(), |guard| !*guard).unwrap();
            *g = false;
        });

    }
}

#[cfg(test)] mod test {
    #[test] fn test_semaphore() {
        dlog::context::Context::reset();
        let s = super::Semaphore {
            c: std::sync::Condvar::new(),
            m: std::sync::Mutex::new(false)
        };
        s.signal();
        s.wait();
    }
}