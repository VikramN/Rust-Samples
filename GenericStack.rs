struct Stack<T> {
    data:Vec<T>
}

impl<T:std::fmt::Show> Stack<T> {
    pub fn new()->Stack<T>{
        Stack { data : vec!() }
    }
    
    pub fn Push(&mut self, value:T) {
        self.data.push(value);
    }
    
    pub fn Pop(&mut self)->Option<T> {
        self.data.pop()
    }
    
    pub fn Print(&self){
        for i in self.data.iter() {
            println!("{}", i);
        }
    }
}


fn main() {
    let mut stack: Stack<String> = Stack::new();
    
    for i in range(10i,20) {
        stack.Push(i.to_string());
    }
    
    stack.Print();
    
    for i in range(1i,5) {
        println!("Popped: {}",  stack.Pop());
    }
    
    for i in range(1i,5) {
        stack.Push(i.to_string());
    }
    
    stack.Print();
}


