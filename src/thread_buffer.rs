use std::{sync::{Arc, Mutex}, collections::VecDeque, time::Duration};

#[derive(Debug)]
pub enum BuffError {
    Full,
    Empty,
}

#[derive(Clone, Copy)]
pub enum BuffSize {
    Finite(usize),
    Infinite,
}

struct Channel<T> {
    buffer: Arc<Mutex<Vec<VecDeque<T>>>>,
    capacity: BuffSize,
    lengths: Vec<usize>,
}

impl<T: Copy> Channel<T> {
    pub fn new(capacity: BuffSize) -> Self {
        Self { buffer: Arc::new(Mutex::new(Vec::new())), capacity, lengths: Vec::new() }
    }
    
    pub fn spawn_endpoint(&mut self) -> () {
        
    }
}

struct Buffer<T> {
    data: VecDeque<(T, usize)>, 
    capacity: BuffSize,
}

impl<T> Buffer<T> {
    pub fn new(capacity: BuffSize) -> Self{
        Self { data: VecDeque::new(), capacity }
    }

    pub fn try_write(&mut self, data: T) -> Result<(), BuffSize> {
        if let BuffSize::Finite(size) = self.capacity {
            for buffer in self.data {
                if buff
            }
        }
    }
}

struct ThreadBuffer<T>(Arc<Mutex<VecDeque<T>>>, BuffSize);

impl<T> ThreadBuffer<T> {
    pub fn new(buffer: &Arc<Mutex<VecDeque<T>>>, capacity: BuffSize) -> Self {
        Self(Arc::clone(buffer), capacity)
    }
    
    pub fn write(&mut self, data: T) {
        while let Err(_) = self.try_write(&data) {
            std::thread::sleep(Duration::from_micros(10));
        }
    }

    pub fn read(&mut self) -> T {
        let mut result = self.try_read();

        while let Err(_) = result {
            std::thread::sleep(Duration::from_micros(10));
            result = self.try_read();
        }

        return result.unwrap();
    }

    pub fn try_write(&mut self, data: &T) -> Result<(), BuffError> {
        if let BuffSize::Finite(capacity) = self.1 {
            let mut buffer = self.0.lock().unwrap();

            if buffer.len() < capacity {
                Ok(buffer.push_back(*data))
            } else {
                Err(BuffError::Full)
            }
        } else {
            Ok(self.0.lock().unwrap().push_back(*data))
        }
    }

    pub fn try_read(&mut self) -> Result<T, BuffError> {
        if let Some(data) = self.0.lock().unwrap().pop_front() {
            Ok(data)
        } else {
            Err(BuffError::Empty)
        }
    }
}

