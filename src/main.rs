use std::time::Duration;
use channels::{thread_buffer::BuffSize, channel::Channel};

fn main() {
    let channel = Channel::new(BuffSize::Finite(10));

    let mut buffer_1 = channel.spawn_endpoint();
    let mut buffer_2 = channel.spawn_endpoint();

    std::thread::spawn(move || {
        for _ in 0..10 {
            buffer_1.write(10);
            std::thread::sleep(Duration::from_millis(200));
        }
    });

    let thread = std::thread::spawn(move || {
        for _ in 0..10 {
            let incoming = buffer_2.read();

            if incoming == 10 {
                println!("Buffer 2 got data!");
            }
        }
    });

    thread.join().expect("Could not join up with the thread :(");
}
