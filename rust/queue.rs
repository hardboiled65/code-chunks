// The goal: learn how to handle with generics, traits, primitive type array

const MAX: usize = 5;

struct Queue<T: Default + Copy> {
    data: [T; MAX],
    // capacity: usize,
    length: usize,
}

enum Error {
    QueueIsFull,
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Queue is full")
    }
}

impl<T: Default + Copy> Queue<T> {
    fn new(/*capacity: usize*/) -> Queue<T> {
        Queue::<T> {
            data: [Default::default(); MAX],
            length: 0,
        }
    }

    fn enqueue(&mut self, value: T) -> Result<(), Error> {
        let idx = self.length;
        if idx < MAX {
            self.data[idx] = value;
            self.length += 1;
            Ok(())
        } else {
            Err(Error::QueueIsFull)
        }
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.length > 0 {
            let value = self.data[0];
            for idx in 1..self.length {
                self.data[idx - 1] = self.data[idx];
            }
            self.length -= 1;
            Some(value)
        } else {
            None
        }
    }
}

fn main() {
    let mut q = Queue::<i32>::new();
    for i in 1..8 {
        let val = i * 100;
        match q.enqueue(val.clone()) {
            Ok(_) => { println!("{} inserted", val); },
            Err(err) => { println!("cannot insert {}. {:?}", val, err); },
        }
    }
    for _ in 0..7 {
        println!("{}", match q.dequeue() {
            Some(val) => val,
            None => 72,
        });
    }
}
