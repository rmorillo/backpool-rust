pub struct LookBehindPool<T>{
    capacity: u64,
    pool: Vec<T>,
}

impl<T> LookBehindPool<T>{
    pub fn new(capacity: usize) -> LookBehindPool<T> {
        let m: Vec<T> = Vec::new();

        LookBehindPool { capacity: capacity as u64, pool: m}
    }

    pub fn item(&self, index: usize) -> &T{
        &self.pool[index]
    }

    pub fn write(&mut self, value: T){
        if self.pool.len()==self.capacity as usize
            {
                self.pool[0]= value
            }
            else
            {
                self.pool.push(value)
            }
    }

}

