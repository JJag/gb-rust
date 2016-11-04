
fn main() {
    println!("Hello world");
    let mut reg = Register{
        a: 0, f: 0,
        b: 0, c: 0,
        d: 0, e: 0,
        h: 0, l: 0,
        sp: 0, pc: 0,
    };

    reg.ld_b(4);

    println!("{:?}", reg);
}

#[derive(Debug)]
struct Register {
    a: u8,
    f: u8,

    b: u8,
    c: u8,

    d: u8,
    e: u8,

    h: u8,
    l: u8,

    sp: u16,
    pc: u16,
}

impl Register {

    fn ld_b(&mut self, n: u8) {
        self.b = n
    }
}
