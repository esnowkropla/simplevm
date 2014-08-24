use std::io::{File};
use std::slice;
use std::fmt;
use std::num::FromPrimitive;

#[deriving(Show, FromPrimitive)]
enum OPCode {
    NOP = 0x00,
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

impl fmt::Show for VM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
               "Stack Pointer: {}\nInstruction Pointer: {}\nRegisters: {}\nInstructions: {}",
               self.sp,
               self.ip, 
               self.register.as_slice(), 
               self.instructions.slice(self.ip,self.ip+10))
    }
}

fn new() -> VM {
    VM{sp:0, ip:0, register:[0, ..16], stack:[0, ..256], instructions:[0, ..65536]}
}

impl VM {
    fn push(&mut self) -> () {
        if self.sp >= 254 { fail!("Stack overflow at byte {}", self.ip); }
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
            fail!("Attempted to pop empty stack at byte {}", self.ip);
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

    fn jump(&mut self) -> () {
        let mut dest = self.instructions[self.ip+1] as uint;
        dest += self.instructions[self.ip+2] as uint << 8;
        self.ip = dest;
    }

    fn jz(&mut self) -> () {
        let tos = self.pop();
        if tos == 0 {
            self.jump();
        } else {
            self.ip += 3;
        }
    }

    fn jnz(&mut self) -> () {
        let tos = self.pop();
        if tos != 0 {
            self.jump();
        } else{
            self.ip += 3;
        }
    }

    fn add(&mut self) -> () {
        let s1 = self.pop();
        let s2 = self.pop();
        self.stack[self.sp] = s1 + s2;
        self.sp += 1;
        self.ip += 1;
    }

    fn sub(&mut self) -> () {
        let s1 = self.pop();
        let s2 = self.pop();
        self.stack[self.sp] = s2 - s1;
        self.sp += 1;
        self.ip += 1;
    }

    fn mul(&mut self) -> () {
        let s1 = self.pop();
        let s2 = self.pop();
        self.stack[self.sp] = s2 * s1;
        self.sp += 1;
        self.ip += 1;
    }

    fn div(&mut self) -> () {
        let s1 = self.pop();
        let s2 = self.pop();
        self.stack[self.sp] = s2 / s1;
        self.sp += 1;
        self.ip += 1;
    }

    fn print(&mut self) -> () {
        let tos = self.pop();
        println!("{}", tos);
        self.ip += 1;
    }

    fn execute(&mut self) -> () {
        loop {
            println!("{}\nStack: {}\n", self, self.stack.slice(0,10));
            let op: Option<OPCode> = FromPrimitive::from_u8(self.instructions[self.ip]);
            match op {
                Some(code) => match code {
                    NOP => (),
                    PUSH => self.push(), 
                    POP => self.bare_pop(),
                    LOAD => self.load(),
                    STORE => self.store(),
                    JMP => self.jump(),
                    JZ => self.jz(),
                    JNZ => self.jnz(),
                    ADD => self.add(),
                    SUB => self.sub(),
                    MUL => self.mul(),
                    DIV => self.div(),
                    PRINT => self.print(),
                    STOP => break,
                },
                None => println!("Invalid OPCode encountered at {}", self.ip)
            }
        }
    }
}

struct VM  {
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
        Err(e) => println!("Failed to read factorial.bcm with error {}", e),
    }

    x.execute();

}
