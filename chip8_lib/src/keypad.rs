#[repr(usize)]
#[derive(PartialEq, Copy, Clone)]
pub enum KeyPad{
    Num0 = 0,
    Num1 = 1,
    Num2 = 2,
    Num3 = 3,
    Num4 = 4,
    Num5 = 5,
    Num6 = 6,
    Num7 = 7,
    Num8 = 8,
    Num9 = 9,
    A = 10,
    B = 11,
    C = 12,
    D = 13,
    E = 14,
    F = 15,
}

impl KeyPad{
    pub fn all() -> [KeyPad;16]{
        [KeyPad::Num0, KeyPad::Num1, KeyPad::Num2, KeyPad::Num3, KeyPad::Num4, KeyPad::Num5, KeyPad::Num6, KeyPad::Num7, KeyPad::Num8, KeyPad::Num9, KeyPad::A, KeyPad::B, KeyPad::C, KeyPad::D, KeyPad::E, KeyPad::F]
    }

    pub fn to_chip8_str(&self) -> &'static str{
        match self{
            KeyPad::Num0 => "0",
            KeyPad::Num1 => "1",
            KeyPad::Num2 => "2",
            KeyPad::Num3 => "3",
            KeyPad::Num4 => "4",
            KeyPad::Num5 => "5",
            KeyPad::Num6 => "6",
            KeyPad::Num7 => "7",
            KeyPad::Num8 => "8",
            KeyPad::Num9 => "9",
            KeyPad::A => "A",
            KeyPad::B => "B",
            KeyPad::C => "C",
            KeyPad::D => "D",
            KeyPad::E => "E",
            KeyPad::F => "F",
        }
    }

    pub fn to_keyboard_str(&self) -> &'static str{
        match self{
            KeyPad::Num0 => "X",
            KeyPad::Num1 => "1",
            KeyPad::Num2 => "2",
            KeyPad::Num3 => "3",
            KeyPad::Num4 => "Q",
            KeyPad::Num5 => "W",
            KeyPad::Num6 => "E",
            KeyPad::Num7 => "A",
            KeyPad::Num8 => "S",
            KeyPad::Num9 => "D",
            KeyPad::A => "Z",
            KeyPad::B => "C",
            KeyPad::C => "4",
            KeyPad::D => "R",
            KeyPad::E => "F",
            KeyPad::F => "V",
        }
    }
}
