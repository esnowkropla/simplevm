use std::os;
use std::io::{File, BufferedReader};
use std::string::String;

fn main() {
    let args = os::args();
    if args.len() != 2 {
        fail!("Usage: simpleasm infile");
    }

    let path = Path::new(args[1].clone());
    let mut file = BufferedReader::new(File::open(&path));
    let lines : Vec<String> = file.lines().map(|x| x.unwrap()).collect();

    for line in lines.iter() {
        print!("{}", line);
    }
}
