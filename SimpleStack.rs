struct Stack {
    data:Vec<int>
}

impl Stack {
    pub fn new()->Stack{
        Stack { data : vec!() }
    }
    
    pub fn Push(&mut self, value:int) {
        self.data.push(value);
    }
    
    pub fn Pop(&mut self)->Option<int> {
        self.data.pop()
    }
    
    pub fn Print(&self){
        for i in self.data.iter() {
            println!("{}", i);
        }
    }
}


fn main() {
    let mut stack= Stack::new();
    
    for i in range(1i,20) {
        stack.Push(i);
    }
    
    
    stack.Print();
    
    for i in range(1i,10) {
        println!("Popped: {}",  stack.Pop());
    }
    
    for i in range(1i,20) {
        stack.Push(i);
    }
    
    stack.Print();
}


