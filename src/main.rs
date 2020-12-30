// use std::env;
use std::fs;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let file = args.get(0);
    let res = fs::read_to_string("/Users/ouyuan/Desktop/workdir/kongfu/src/test.k").unwrap();
    println!("size ={}",res);
}
