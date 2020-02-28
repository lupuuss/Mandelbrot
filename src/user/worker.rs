use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread::{spawn, JoinHandle};
use std::sync::{Arc, Mutex};

enum InData<T> {
    Function(Box<dyn (Fn() -> T) + Send>),
    Stop()
}

pub struct Worker<T: 'static> {
    thread_senders: Vec<Sender<InData<T>>>,
    occupancy: Vec<Arc<Mutex<i32>>>,
    output_rx: Receiver<T>,
    output_tx: Sender<T>,
    threads_handles: Vec<JoinHandle<()>>,
    verbose: bool
}

impl<T: Send> Worker<T> {

    pub fn new(threads: usize, verbose: bool) -> Worker<T> {

        let (out_tx, out_rx) = channel();


        let mut result = Worker {
            thread_senders: Vec::with_capacity(threads),
            occupancy: Vec::with_capacity(threads),
            output_rx: out_rx,
            output_tx: out_tx,
            threads_handles: Vec::with_capacity(threads),
            verbose: verbose
        };

        result.init();

        return result;
    }

    fn init(&mut self) { 

        for i in 0..self.threads_handles.capacity() {

            let (in_tx, in_rx) = channel::<InData<T>>();
            let out_tx = Sender::clone(&self.output_tx);

            self.thread_senders.push(in_tx);

            self.occupancy.push(Arc::new(Mutex::new(0)));

            let thread_occupancy = Arc::clone(&self.occupancy.last().unwrap());

            let verb = self.verbose;

            if verb { println!("Thread #{} started!", i) }

            let handle = spawn(move || { 
                Worker::thread_loop(in_rx, out_tx, thread_occupancy, verb, i)
             });

            self.threads_handles.push(handle);
        }
        
    }

    fn thread_loop(in_rx: Receiver<InData<T>>, out_tx: Sender<T>, thread_occupancy: Arc<Mutex<i32>>, verb: bool, i: usize) {

        loop {

            let msg = in_rx.recv();

            match msg.unwrap() {

                InData::Function(f) => {

                    out_tx.send(f()).unwrap();

                    let mut busy_counter = thread_occupancy.lock().unwrap();
                    *busy_counter -= 1;

                    if verb { println!("Thread #{} finished work! Occupancy: {}", i, *busy_counter) };
                },
                InData::Stop() => break
            }

        }
        
        if verb {
            println!("Thread #{} stopped!", i);
        }
    }

    pub fn push(&mut self, work: Box<dyn (Fn() -> T) + Send>) {
        
        let n = self.min_occupancy_index();

        let mut picked_thread_occupacy = self.occupancy[n].lock().unwrap();
        *picked_thread_occupacy += 1;

        self.thread_senders[n].send(InData::Function(work)).unwrap();
        if self.verbose { 
            println!("Thread #{} recived work! Occupacy: {}", n, *picked_thread_occupacy)
        };
    }

    fn min_occupancy_index(&self) -> usize {
        
        let mut min_i = 0;
        let mut min_val = *self.occupancy[min_i].lock().unwrap();

        for (i, thread_occupancy) in self.occupancy.iter().enumerate() {
            
            let tmp = *thread_occupancy.lock().unwrap();

            if  tmp < min_val {
                min_i = i;
                min_val = tmp;
            }
        }

        return min_i;
    }

    pub fn is_occupied(&self) -> bool {

        for occupancy_mutex in self.occupancy.iter() {

            if *occupancy_mutex.lock().unwrap() != 0 {
                return true;
            }
        }

        return false;
    }
}

impl<T> Worker<T> {
    pub fn output_receiver(&self) -> &Receiver<T> {
        return &self.output_rx;
    }
    
    pub fn shutdown(&mut self) {

        for sender in self.thread_senders.iter() {
            
            sender.send(InData::Stop()).unwrap();
        }

        loop {

            match self.threads_handles.pop() {
                Some(handle) => handle.join().unwrap(),
                None => break
            }
        }
    }
}

impl<T> Drop for Worker<T> {

    fn drop(&mut self) {
        self.shutdown();
    }
}