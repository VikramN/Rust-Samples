struct Queue<T> {
    data: Vec<T>
}

impl<T:std::fmt::Show> Queue<T> {
    pub fn new()->Queue<T>{
        Queue { data: vec!() }
    }
    
    pub fn EnQueue(&mut self, value:T) {
        self.data.insert(0,value);
    }
    
    pub fn DeQueue(&mut self)->Option<T> {
        self.data.pop()
    }
    
    pub fn Print(&self){
        for i in self.data.iter() {
            println!("{}", i);
        }
    }
}


fn main() {
    let mut q: Queue<int> = Queue::new();
    
    for i in range(0i,10) {
        q.EnQueue(i);
        println!("En_queued {}", i);
    }

    for i in range(0i,11) {
        match q.DeQueue() {
            Some(i) => { println!("De_queued {}", i); }
            None => { println!("Queue is empty"); }
        }
    }
    
    let mut sq: Queue<String> = Queue::new();
    
    for i in range(0i,10) {
        sq.EnQueue(i.to_string());
        println!("En_queued {}", i);
    }

    
    for i in range(0i,11) {
        match sq.DeQueue() {
            Some(i) => { println!("De_queued {}", i); }
            None => { println!("Queue is empty"); }
        }
    }
    
}


