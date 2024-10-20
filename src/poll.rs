use std::{
    io::{self, Result},
    net::TcpStream,
    os::fd::AsRawFd
}
use crate::ffi;

pub struct Poll {
    registry: Registry,
}

impl Poll {
    pub fn new() -> Result<Self> {
        todo!()
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn poll(&mut self, events: &mut Events, timout: Option<i32>) -> Result<()> {
        todo!()
    }
}

pub struct Registry {
    raw_fd: i32,
}

impl Registry {
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