pub struct Queue {
    begin: i32,
    end: i32,
    total: i32,
    values: [i32; 10],
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            begin: 0,
            end: 0,
            total: 0,
            values: [0; 10]
        }
    }

    pub fn insert(&mut self, elem: i32) {
        let index: usize = self.end as usize;
        self.values[index] = elem;
        self.end = (self.end + 1) % 10;
        self.total = self.total + 1;
    }
    pub fn remove(&mut self) -> i32 {
        let index: usize = self.begin as usize;
        let elem: i32 = self.values[index];
        self.begin = (self.begin + 1) % 10;
        self.total = self.total - 1;

        elem
    }
    pub fn is_empty(&self) -> bool {
        self.total == 0
    }
    pub fn is_full(&self) -> bool {
        self.total == 10
    }
}