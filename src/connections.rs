use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::Receiver;

pub struct ConnectionPool {
    handlers: Vec<ConnectionHandler>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct ConnectionHandler {
    id: usize,
    thread: Option<JoinHandle<()>>
}

impl ConnectionPool {

    pub fn new(size: usize) -> Result<ConnectionPool, &'static str> {
        match size > 0 {
            true => {
                let (sender, receiver) = mpsc::channel();
                let receiver = Arc::new(Mutex::new(receiver));

                let mut handlers = Vec::with_capacity(size);

                for id in 0..size {
                    handlers.push(ConnectionHandler::new(id, Arc::clone(&receiver)));
                }

                Ok(ConnectionPool { handlers, sender })
            }
            false => Err("Connection pool size must be bigger than 0.")
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}


impl ConnectionHandler {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> ConnectionHandler {

        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {

                    job();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        ConnectionHandler { id, thread: Some(thread) }
    }
}