use std::env;
mod backpool;

use backpool::LookBehindPool;

struct Prices{
    open: u64,
    high: u64,
    low: u64,
    close: u64
}

fn main(){
    
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
    println!("the thing's total is {}", my_thing.item(message.parse().unwrap()).open);
    println!("the thing's total is {}", my_thing.item(message.parse().unwrap()).high);
    println!("the thing's total is {}", my_thing.item(message.parse().unwrap()).low);
    println!("the thing's total is {}", my_thing.item(message.parse().unwrap()).close);
}
