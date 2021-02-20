use vm::say_hi;

mod rabalder;

use crate::rabalder::hahaha;

fn main() {
    println!("Hello, world!");
    let b: [u8;4] = [1,2,3,4];
    println!("{:#08b}{:#08b}",b[0],b[1]);
    say_hi();
    hahaha();
}
