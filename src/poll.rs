use std::{
    io::{self, Result},
    net::TcpStream,
    os::fd::AsRawFd
}
use crate::ffi;

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
    pub fn poll(&mut self, events: &mut Events, timout: Option<i32>) -> Result<()> {
        todo!()
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