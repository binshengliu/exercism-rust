use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::collections::HashMap;

pub fn frequency(input: &[&'static str], size: usize) -> HashMap<char, usize> {
    let (job_sender, job_receiver) = mpsc::channel();
    let job_receiver = Arc::new(Mutex::new(job_receiver));

    let (result_sender, result_receiver) = mpsc::channel();
    let result_sender = Arc::new(Mutex::new(result_sender));

    let mut workers = Vec::new();
    for id in 0..size {
        workers.push(Worker::new(
            id,
            job_receiver.clone(),
            result_sender.clone(),
        ));
    }

    for s in input {
        job_sender.send(Message::Job(s)).unwrap();
    }

    let mut result = HashMap::new();
    for _ in input {
        let map = result_receiver.recv().unwrap();
        for (k, v) in map.into_iter() {
            *result.entry(k).or_insert(0) += v;
        }
    }

    for _ in &workers {
        job_sender.send(Message::Terminate).unwrap();
    }

    for worker in workers {
        worker.thread.join().unwrap();
    }
    result
}

fn stats(input: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in input.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}

enum Message {
    Job(&'static str),
    Terminate,
}

struct Worker {
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(
        _id: usize,
        job_receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        job_sender: Arc<Mutex<mpsc::Sender<HashMap<char, usize>>>>,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = job_receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::Job(s) => {
                    job_sender.lock().unwrap().send(stats(s)).unwrap()
                }
                Message::Terminate => {
                    // println!("Worker {} quits.", id);
                    break;
                }
            }
        });
        Worker { thread }
    }
}
