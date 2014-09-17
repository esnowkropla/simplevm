extern crate simplevm;

use simplevm::VM;
use std::os;
use std::io::{File};

fn main() {
    let args = os::args();
    if args.len() != 2 {
        fail!("Usage: simplevm infile");
    }

    let path = Path::new(args[1].clone());

    let mut x  = VM::new();
    match File::open(&path).read_to_end() {
        Ok(contents) => x.program(contents.as_slice()),
        Err(e) => fail!("Failed to read factorial.bcm with error {}", e),
    }

    x.execute();
}
