# blocking_semaphore

![logo](art/logo.png)

A simple and performant blocking semaphore.

todo list:
- [x] binary semaphore
- [ ] counting semaphore
- [x] portable stdlib implementation with [Condvar](https://doc.rust-lang.org/std/sync/struct.Condvar.html)
- [ ] optimize Windows performance with [WaitOnAddress](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitonaddress)
- [ ] optimize Linux performance with [futex](https://man7.org/linux/man-pages/man2/futex.2.html)
- [ ] optimize macOS performance with dispatch_semaphore