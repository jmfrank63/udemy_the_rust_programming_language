use bitflags::bitflags;
use std::fmt;

bitflags! {
    struct Flags: u8 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
        const D = 0b00001000;
        const ABCD = Self::A.bits | Self::B.bits | Self::C.bits | Self::D.bits;
    }
}

impl Flags {
    pub fn clear(&mut self) -> &mut Flags {
        self.bits = 0;
        self
    }

    pub fn set_all(&mut self) -> &mut Flags {
        self.bits = std::u8::MAX;
        self
    }

    pub fn invert(&mut self) -> &mut Flags {
        self.bits = !self.bits;
        self
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08b}", self.bits)
    }
}

pub fn play_with_bits() {
    let a = Flags::A;
    let b = Flags::B;
    let c = Flags::C;
    let d = Flags::D;

    println!("{:?}", a);

}
