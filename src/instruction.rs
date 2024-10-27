use bitfield_struct::bitfield;

#[repr(u16)]
#[derive(Debug, PartialEq)]
pub(crate) enum Code {
    NULL    = 0b0000,
    STOP    = 0b0001,
    LOAD    = 0b0010,
    STORE   = 0b0011,
    JUMP    = 0b0100,
    JNEG    = 0b0101,
    JZERO   = 0b0110,
    ADD     = 0b1000,
    SUB     = 0b1001,
    AND     = 0b1100,
    OR      = 0b1101,
    NOT     = 0b1110,
    XOR     = 0b1111,
    SHL     = 0b1010,
    SHR     = 0b1011
}

#[repr(u16)]
#[derive(Debug)]
pub(crate) enum Adrt {
    DOT     = 0b00,
    AT      = 0b01,
    STAR    = 0b10,
    PLUS    = 0b11,
}

#[bitfield(u16, order = Msb)]
pub(crate) struct InnerInstruction {
    #[bits(1)] pub(crate) sign: u16,
    #[bits(4)] pub(crate) code: Code,
    #[bits(2)] pub(crate) adrt: Adrt,
    #[bits(9)] pub(crate) adr: u16,
}
impl InnerInstruction {
    pub(crate) fn adr_val(&self) -> i16 {
        if self.sign() == 1 {
            self.adr() as i16 * -1
        } else {
            self.adr() as i16
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) union Instruction {
    pub(crate) inner: InnerInstruction,
    pub(crate) raw: u16
}

impl Adrt {
    const fn into_bits(self) -> u16 {
        self as _
    }
    const fn from_bits(value: u16) -> Self {
        match value {
            0b00 => Adrt::DOT,
            0b01 => Adrt::AT,
            0b10 => Adrt::STAR,
            0b11 => Adrt::PLUS,
            _ => panic!("Wrong value passed to ADRT"),
        }
    }
}


impl Code {
    const fn into_bits(self) -> u16 {
        self as _
    }
    const fn from_bits(value: u16) -> Self {
        match value {
            0b0000 => Code::NULL,
            0b0001 => Code::STOP,
            0b0010 => Code::LOAD,
            0b0011 => Code::STORE,
            0b0100 => Code::JUMP,
            0b0101 => Code::JNEG,
            0b0110 => Code::JZERO,
            0b1000 => Code::ADD,
            0b1001 => Code::SUB,
            0b1010 => Code::SHL,
            0b1011 => Code::SHR,
            0b1100 => Code::AND,
            0b1101 => Code::OR,
            0b1110 => Code::NOT,
            0b1111 => Code::XOR,
            _ => panic!("Wrong value passed to CODE"),
        }
    }
}