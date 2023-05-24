use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use crate::thread_buffer::{ThreadBuffer, BuffSize};

pub struct Channel<T>(Arc<Mutex<VecDeque<T>>>, BuffSize);

impl<T: Copy> Channel<T> {
    pub fn new(capacity: BuffSize) -> Self {
        let buffer = match capacity {
            BuffSize::Finite(capacity) => VecDeque::with_capacity(capacity),
            BuffSize::Infinite => VecDeque::new(),
        };

        Self(Arc::new(Mutex::new(buffer)), capacity)
    }
    
    // spawns a new threadbuffer to write to and read from
    pub fn spawn_endpoint(&self) -> ThreadBuffer<T> {
        ThreadBuffer::new(&self.0, self.1)
    }
}

