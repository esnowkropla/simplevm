use std::io::{File, IoResult};
use std::slice;
use std::fmt;

#[deriving(Show)]
enum OPCode {
	NOP,
	PUSH,
	POP,
	LOAD,
	STORE,
	JMP,
	JZ,
	JNZ,
	ADD,
	SUB,
	MUL,
	DIV,
	PRINT,
	STOP
}

impl fmt::Show for vm {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Stack Pointer: {}\nInstruction Pointer: {}\n", self.sp, self.ip);
		write!(f, "Registers: {}\n", self.register.as_slice());
		write!(f, "Instructions: {}", self.instructions.slice(self.ip,self.ip+10))
	}
}

fn new() -> vm {
	vm{sp:0, ip:0, register:[0, ..16], stack:[0, ..256], instructions:[0, ..65536]}
}

impl vm {
	fn push(&mut self) -> () {
		self.stack[self.sp] = 0;
		for i in range(0, 4) {
			self.stack[self.sp] += self.instructions[self.ip+1+i] as i32 << 8*i;
		}
		self.sp += 1;
		self.ip += 5;
	}

	fn bare_pop(&mut self) -> () {
		let _ = self.pop();
	}

	fn pop(&mut self) -> i32 {
		if self.sp > 0 {
			self.sp -= 1;
		} else {
			fail!("Attempted to pop empty stack at line {}", self.ip);
		}
		self.stack[self.sp]
	}

	fn load(&mut self) -> () {
		let r = self.instructions[self.ip+1];
		self.stack[self.sp] = self.register[r as uint];
		self.sp += 1;
		self.ip += 2;
	}

	fn store(&mut self) -> () {
		let r = self.instructions[self.ip + 1];
		self.register[r as uint] = self.pop();
		self.ip += 2;
	}


	fn execute(&mut self) -> () {
		let mut i = 0u;
		loop {
			println!("{}", self);
			println!("Stack: {}\n", self.stack.slice(0,10));
			match self.instructions[self.ip] {
				0 => (),
				1 => self.push(), 
				2 => self.bare_pop(),
				3 => self.load(),
				4 => self.store(),
				x => println!("Encountered {}", x)
			}
			i += 1;
			if i > 5 {break;}
		}
	}
}
				

struct vm  {
	sp : uint,
	ip : uint,
	register : [i32, ..16],
	stack : [i32, ..256],
	instructions : [u8, ..65536],
}

fn main() {
	let mut x  = new();
	match File::open(&Path::new("factorial.bcm")).read_to_end() {
		Ok(contents) => slice::bytes::copy_memory(x.instructions, contents.as_slice()),
		Err(e) => println!("Failed to read factorial.bcm"),
	}

	x.execute();

}
