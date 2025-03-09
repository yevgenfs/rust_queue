use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use std::sync::Arc;

const QUEUE_CAPACITY: usize = 6;

struct Queue<T: Copy + Default> {
    data: [T; QUEUE_CAPACITY],
    front: usize,
    rear: usize,
}

#[derive(PartialEq)]
enum QueueErr {
    Ok,
    Full,
    Empty,
}

impl<T: Copy + Default> Queue<T> {
    fn new( ) -> Self {
        Queue {
            data: [T::default(); QUEUE_CAPACITY],
            front: 0,
            rear: 0,
        }
    }

    fn is_full(&self) -> bool {
        return self.rear == QUEUE_CAPACITY;
    }

    fn enqueue(&mut self, item: T) -> QueueErr {
        if self.is_full() {
            return QueueErr::Full;
        }
        
        self.data[self.rear] = item;
        self.rear += 1;
        return QueueErr::Ok;
    }

    fn dequeue(&mut self, value : &mut T) -> QueueErr {
        if self.is_empty() {
            *value = T::default();
            return QueueErr::Empty;
        } 

        *value = self.data[self.front];
        self.front += 1;

        if self.front == self.rear {
            self.front = 0; 
            self.rear  = 0;
        }

        return QueueErr::Ok;
    }

    fn is_empty(&self) -> bool {
        return self.rear == 0;
    }
}



#[tokio::main]
async fn main() {
    let queue = Arc::new(Mutex::new(Queue::<i32>::new()));

    let producer_queue = queue.clone();
    let consumer_queue = queue.clone();

    let producer = tokio::spawn(async move{
        for i in 1..=10 {
            loop {
                let mut queue = producer_queue.lock().await;
                if !queue.is_full() {
                    queue.enqueue(i);
                    println!("Produced: {}", i);
                    break;
                } else {
                    drop(queue);
                    sleep(Duration::from_millis(50)).await; 
                }
            }
            sleep(Duration::from_millis(100)).await; 
        }
    });

    let consumer = tokio::spawn(async move{
        loop {
            let mut queue = consumer_queue.lock().await;
            let mut value : i32 = 0;
            if QueueErr::Ok == queue.dequeue(&mut value) {
                println!("Consuming: {}", value);
                drop(queue);
                sleep(Duration::from_millis(500)).await; 
            } else {
                drop(queue);
                sleep(Duration::from_millis(50)).await; 
            }
        }
    });

    let _ = tokio::join!(producer, consumer);
}