#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Key {
    A,          //  0
    B,          //  1
    C,          //  2
    D,          //  3
    E,          //  4
    F,          //  5
    G,          //  6
    H,          //  7
    I,          //  8
    J,          //  9
    K,          // 10
    L,          // 11
    M,          // 12
    N,          // 13
    O,          // 14
    P,          // 15
    Q,          // 16
    R,          // 17
    S,          // 18
    T,          // 19
    U,          // 20
    V,          // 21
    W,          // 22
    X,          // 23
    Y,          // 24
    Z,          // 25

    // 26-35
    Key0,       // 26
    Key1,       // 27
    Key2,       // 28
    Key3,       // 29
    Key4,       // 30
    Key5,       // 31
    Key6,       // 32
    Key7,       // 33
    Key8,       // 34
    Key9,       // 35

    // 36-47
    Minus,      // 36
    Plus,       // 37
    Equals,     // 38
    LBracket,   // 39
    RBracket,   // 40
    Period,     // 41
    Comma,      // 42
    Colon,      // 43
    Semicolon,  // 44
    Apostrophe, // 45
    Backslash,  // 46
    Slash,      // 47

    // 48-54
    Tab,        // 48
    Escape,     // 49
    Space,      // 50
    Back,       // 51
    Delete,     // 52
    Return,     // 53
    Shift,      // 54

    // 55-58
    ArrowUp,    // 55
    ArrowDown,  // 56
    ArrowLeft,  // 57
    ArrowRight, // 58
}

impl Key {
    pub fn if_letter_get(&self, shift: bool) -> Option<char> {
        if (*self as u8) < 26 {
            return Some((*self as u8 + 65) as char);
        }

        if shift {
            return match self {
                Self::Minus => Some('_'),
                Self::Equals => Some('+'),
                Self::LBracket => Some('{'),
                Self::RBracket => Some('}'),
                Self::Period => Some('>'),
                Self::Comma => Some('<'),
                Self::Semicolon => Some(':'),
                Self::Apostrophe => Some('"'),
                Self::Backslash => Some('|'),
                Self::Slash => Some('?'),
                _ => None,
            };
        }

        match self {
            Self::Minus => Some('-'),
            Self::Plus => Some('+'),
            Self::Equals => Some('='),
            Self::LBracket => Some('['),
            Self::RBracket => Some(']'),
            Self::Period => Some('.'),
            Self::Comma => Some(','),
            Self::Colon => Some(':'),
            Self::Semicolon => Some(';'),
            Self::Apostrophe => Some('\''),
            Self::Backslash => Some('\\'),
            Self::Slash => Some('/'),
            Self::Space => Some(' '),
            _ => None,
        }
    }
}

pub fn is_key_down(input: u64, key: Key) -> bool {
    let key_id = key as u8;
    (input & (1 << key_id)) > 0
}

pub fn input_to_vec(input: u64) -> Vec<Key> {
    let mut result = Vec::new();
    iter_set_bits(input, |id| result.push(unsafe { std::mem::transmute::<u8, Key>(id as u8) }));
    result
}

fn iter_set_bits(mut bitset: u64, mut on_bit_set: impl FnMut(usize)) {
    while bitset != 0 {
        let t = bitset & bitset.wrapping_neg();
        on_bit_set(bitset.trailing_zeros() as usize);
        bitset ^= t;
    }
}
