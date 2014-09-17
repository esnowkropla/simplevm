use std::os;
use std::io::{File, BufferedReader};

fn main() {
    let args = os::args();
    if args.len() != 2 {
        fail!("Usage: simpleasm infile");
    }

    let path = Path::new(args[1].clone());
    let mut file = BufferedReader::new(File::open(&path));
    let raw = match file.read_to_string() {
        Ok(r) => r,
        Err(e) => fail!("Couldn't read program {}, got error \"{}\"", path.display(), e)
    };

    println!("{}",raw);
}
