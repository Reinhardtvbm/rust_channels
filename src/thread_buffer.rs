use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

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

/// A buffer for communication between two threads:
///
/// It holds a vector that can only hold a maximum number of items `capacity`
/// 
/// Examples:
/// ```rust
///     // for ThreadBuffer<i32>
///     let new_data: i32 = 30;
///     
///     // just try to write the data untill the buffer is no longer full
///     while let Err(e) = thread_buffer.write(new_data) {
///         // log the error
///         println!("{:?}", e); 
///         std::thread::sleep(Duration::from_micros(10));
///     }
/// ```
pub struct ThreadBuffer<T> {
    data: Arc<Mutex<VecDeque<T>>>,
    capacity: BuffSize,
}

impl<T: Copy> ThreadBuffer<T> {
    pub fn new(thread_buffer: &Arc<Mutex<VecDeque<T>>>, capacity: BuffSize) -> Self {
        Self { data: Arc::clone(thread_buffer), capacity }
    }
    
    pub fn enqueue(&mut self, item: T) -> Result<(), BuffError> {
        let mut buffer = self.data.lock().unwrap();

        if let BuffSize::Finite(capacity) = self.capacity {
            if buffer.len() <= capacity {
                Ok(buffer.push_back(item))
            } else {
                Err(BuffError::Full)
            }
        } else {
            Ok(buffer.push_back(item))
        }
    }

    pub fn dequeue(&mut self) -> Result<T, BuffError> {
        let mut buffer = self.data.lock().unwrap();

        match buffer.pop_front() {
            Some(item) => Ok(item),
            None => Err(BuffError::Full),
        }
    }

    pub fn write(&mut self, item: T) {
        while let Err(_) = self.enqueue(item) {
            std::thread::sleep(Duration::from_micros(100));
        }
    }

    pub fn read(&mut self) -> T {
        let mut result = self.dequeue();

        while let Err(_) = result {
            std::thread::sleep(Duration::from_micros(100));
            result = self.dequeue();
        }

        result.unwrap()
    }
}
