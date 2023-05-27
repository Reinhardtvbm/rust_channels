use std::sync::{Arc, Mutex};
use std::collections::{VecDeque, HashMap};

use crate::thread_buffer::{ThreadBuffer, BuffSize};

type IndependantBuffers<K, T> = HashMap<K, VecDeque<T>>;

pub struct Channel<K, T>(Arc<Mutex<IndependantBuffers<K, T>>>, BuffSize);

impl<K, T: Copy> Channel<K, T> {
    pub fn new(capacity: BuffSize) -> Self {
        let buffers = HashMap::new();
        Self(Arc::new(Mutex::new(buffers)), capacity)
    }
    
    // spawns a new threadbuffer to write to and read from
    pub fn spawn_endpoint(&self) -> ThreadBuffer<T> {
        ThreadBuffer::new(&self.0, self.1)
    }
}

