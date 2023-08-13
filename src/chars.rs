const CHAR_MAPPING: [u16; 47] = [
    0x33, //,
    0x0C, //-
    0x34, //.
    0x35, //\/
    0x0B, //0
    0x02, //1
    0x03, //2
    0x04, //3
    0x05, //4
    0x06, //5
    0x07, //6
    0x08, //7
    0x09, //8
    0x0A, //9
    0x0, 0x27, //;
    0x0, 0x0D, //=
    0x0, 0x0, 0x0, 0x1E, //A
    0x30, //B
    0x2E, //C
    0x20, //D
    0x12, //E
    0x21, //F
    0x22, //G
    0x23, //H
    0x17, //I
    0x24, //J
    0x25, //K
    0x26, //L
    0x32, //M
    0x31, //N
    0x18, //O
    0x19, //P
    0x10, //Q
    0x13, //R
    0x1F, //S
    0x14, //T
    0x16, //U
    0x2F, //V
    0x11, //W
    0x2D, //X
    0x15, //Y
    0x2C, //Z
];

#[derive(Debug)]
pub enum DXCode {
    Symbol(u16),
    Shifted(u16),
}

/**
   Convert ASCII char into DirectX key code
*/
pub fn char_to_dxcodes(c: char) -> Option<DXCode> {
    let mut c_u8 = c as u8;

    if c.is_ascii_lowercase() {
        c_u8 &= 0xdf;
    }

    if c.is_ascii_whitespace() {
        return Some(DXCode::Symbol(0x39));
    }

    if c == ":".chars().next().unwrap() {
        return Some(DXCode::Shifted(0x27));
    }

    if c_u8 < 0x5B && c_u8 > 0x2B {
        let index = c_u8 - 0x2C;
        let code = CHAR_MAPPING[index as usize];
        // println!("{} {}", index, code);
        if code == 0x0 {
            None
        } else if c.is_ascii_uppercase() {
            // Press SHIFT
            Some(DXCode::Shifted(code))
        } else {
            Some(DXCode::Symbol(code))
        }
    } else {
        None
    }
}
