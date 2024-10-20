pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLLIN: i32 = 0x1; //read ops bitflag
pub const EPOLLET: i32 = 1 << 31; // edge triggered mode bitflag

#[link(name = "C")]
extern "C" {
    // Create an epoll queue 
    pub fn epoll_create(size: i32) -> i32;

    // close file descriptor
    pub fn close(fd: i32) -> i32;

    /**
     * epoll control interface
     * epfd: file descriptor
     * op: operation we want to perform (add, modify, delete)
     * fd: 
     * events: what kind of events we want to be notified
     */
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;

    /**
     * This call blocks the thread and waits until we get a notification or timeout
     */
    pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}

#[derive(Debug)]
//linux expects packed data here.
#[repr(C, packed)]
pub struct Event {
    pub(crate) events: u32, //bitmask
    //token to identify event
    pub(crate) epoll_data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}