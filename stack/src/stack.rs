pub struct Stack {
    len: usize,
    values: [i32; 10],
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            len: 0,
            values: [0; 10]
        }
    }
    
    pub fn push(&mut self, val: Option<i32>) {
        if !self.is_full() {
            self.values[self.len] = val.unwrap_or(0);
            self.len += 1;
        }
    }
    
    pub fn pop(&mut self) -> Option<i32> {
        if !self.is_empty() {
            self.len -= 1;
            Some(self.values[self.len])
        } else {
            None
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    pub fn is_full(&self) -> bool {
        self.len == 10
    }
}