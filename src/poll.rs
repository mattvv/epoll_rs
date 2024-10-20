use std::{
    io::{self, Result},
    net::TcpStream,
    os::fd::AsRawFd
}
use crate::ffi;

type Events = Vec<ffi::Event>;

/**
 * Represents Event Queue
 */
pub struct Poll {
    registry: Registry,
}

impl Poll {
    /**
     * Create a new event queue
     */
    pub fn new() -> Result<Self> {
        let res = unsafe { ffi::epoll_create(1)};
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            registry: Registry { raw_fd: res }
        })
    }

    /**
     * Returns a reference to the registry we can use to register interest aboutb new events
     */
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /**
     * Blocks the thread it's called on until an event is ready or it times out
     */
    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        // get file descriptor
        let fd = self.registry.raw_fd;
        let timeout = timeout.unwrap_or(-1);
        let max_events = events.capacity() as i32;

        let res = unsafe { ffi::epoll_wait(fd, events.as_mut_ptr(), max_events, timeout)};

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        unsafe { events.set_len(res as usize) };
        Ok(());
    }
}

/**
 * Allows us to register new events on the system
 */
pub struct Registry {
    raw_fd: i32,
}

impl Registry {
    /**
     * Mimic's mio's register schema.
     */
    pub fn register(&self, source: &TcpStream, token: usize, interests: i32) -> Result<()> 
    {
        todo!()
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        todo!()
    }
}