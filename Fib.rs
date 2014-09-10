struct Fib {
    fib_cache: Vec<uint>
}

impl Fib {

    pub fn new()->Fib{
        Fib{ fib_cache: vec!(1,1,1) }
    }
    
    pub fn calc(&mut self, n:uint) ->uint{
        if n < 3 {
            1
        } else {
            if n < self.fib_cache.len() - 1 {
                return self.fib_cache[n]
            }
            
            let value =  self.calc(n-1)+self.calc(n-2);
            self.fib_cache.insert(n,value);
            self.fib_cache[n]
        }
    }
}

// This code is editable and runnable!
fn main() {

    let mut f =  Fib::new();
    
    for i in range(1u, 120) {
        println!("{} ", f.calc(i));
    }
    
}