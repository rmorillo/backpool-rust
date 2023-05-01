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

#[cfg(test)]

struct Prices{
    open: u64,
    high: u64,
    low: u64,
    close: u64
}

#[test]
fn it_works() {
    let input: usize = 0;

    println!("value={}",input);
    let mut my_thing = LookBehindPool::<Prices>::new(2);
    my_thing.write(Prices{open: 1, high:2, low:3, close: 4});
    my_thing.write(Prices{open: 1, high:6, low:3, close: 4});
    //my_thing.write(Prices{open: 1, high:5, low:3, close: 4});
    println!("the thing's total is {}", my_thing.item(input).open);
    println!("the thing's total is {}", my_thing.item(input).high);
    println!("the thing's total is {}", my_thing.item(input).low);
    println!("the thing's total is {}", my_thing.item(input).close);

    assert_eq!(2 + 2, 4);
}
