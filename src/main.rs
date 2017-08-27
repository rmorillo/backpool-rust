use std::env;

fn main(){
    struct Prices{
        open: u64,
        high: u64,
        low: u64,
        close: u64
    }

    struct LookBehindPool<T>{
        capacity: u64,
        pool: Vec<T>,
    }

    impl<T> LookBehindPool<T>{
        fn new(capacity: usize) -> LookBehindPool<T> {
            let m: Vec<T> = Vec::new();

            LookBehindPool { capacity: capacity as u64, pool: m}
        }

        fn item(&self, index: usize) -> &T{
            &self.pool[index]
        }

        fn write(&mut self, value: T){
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

    let mut args = env::args();

    let message = match args.nth(1) {
        Some(x) => x,
        None => panic!()
    };
    let input: usize = message.parse().unwrap();

    println!("value={}",input);
    let mut my_thing = LookBehindPool::<Prices>::new(2);
    my_thing.write(Prices{open: 1, high:2, low:3, close: 4});
    my_thing.write(Prices{open: 1, high:6, low:3, close: 4});
    //my_thing.write(Prices{open: 1, high:5, low:3, close: 4});
    println!("the thing's total is {}", my_thing.item(0).open);
    println!("the thing's total is {}", my_thing.item(0).high);
    println!("the thing's total is {}", my_thing.item(0).low);
    println!("the thing's total is {}", my_thing.item(0).close);
}