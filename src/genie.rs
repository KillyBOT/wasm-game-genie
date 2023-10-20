use wasm_bindgen::prelude::*;

const NES_CHARS: &str = "APZLGITYEOXUKSVN";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GGCode {
    NESCode{ addr: u16, val: u8},
    NESCondCode{ addr: u16, cond: u8, val: u8}
}

fn nes_char_to_num(c: char) -> Option<u32> {
    match NES_CHARS.find(c) {
        Some(p) => Some(p as u32),
        None => None
    }
}

fn nes_num_to_char(b: u32) -> Option<char> {
    NES_CHARS.chars().nth(b as usize)
}

fn valid_nes_code(code: &str) -> bool {
    if code.len() != 6 && code.len() != 8 {
        return false;
    }
    
    for c in code.chars() {
        if nes_char_to_num(c) == None {
            return false;
        }
    }

    return true;
}

fn decode_nes(code: &str) -> Option<GGCode> {
    // First, validate that the code is correct
    if !valid_nes_code(code) {
        return None;
    }
    
    let vals = code.chars()
        .map(|c| nes_char_to_num(c).unwrap())
        .collect::<Vec<u32>>();

    let addr = (0x8000 + ((vals[3] & 7) << 12)
                    | ((vals[5] & 7) << 8) | ((vals[4] & 8) << 8)
                    | ((vals[2] & 7) << 4) | ((vals[1] & 8) << 4)
                    |  (vals[4] & 7)       |  (vals[3] & 8)) as u16;

    match code.len() {
        6 => {
            let val = (((vals[1] & 7) << 4) | ((vals[0] & 8) << 4)
                            | (vals[0] & 7) | (vals[5] & 8)) as u8;

            Some(GGCode::NESCode { addr, val })
        },
        8 => {
            let cond = (((vals[7] & 7) << 4) | ((vals[6] & 8) << 4)
                            | (vals[6] & 7) | (vals[5] & 8)) as u8;
            let val = (((vals[1] & 7) << 4) | ((vals[0] & 8) << 4)
                        | (vals[0] & 7) | (vals[7] & 8)) as u8;

            Some(GGCode::NESCondCode { addr, cond, val})
        },
        _ => None
    }
}

pub fn get_alternate_code_nes(code: &str) -> Option<String> {
    if !valid_nes_code(code) {
        return None;
    }

    let alternate = String::from(code);

    // Flip msb of 3rd char
    let b = nes_char_to_num(code.chars().nth(2).unwrap()).unwrap() ^ 8;
    // This is so stupid
    Some(alternate.chars().enumerate()
        .map(
            |(i, c)| 
            if i == 2 {
                nes_num_to_char(b).unwrap()
            } else {
                c
            })
        .collect()
    )
}

pub fn encode(code: GGCode) -> String {
    let mut encoded: String = String::with_capacity(8);
    match code {
        GGCode::NESCode { addr, val } => {
            let addr: u32 = addr as u32;
            let val: u32 = val as u32;
            encoded.push(nes_num_to_char(
                ((val >> 4) & 8) | (val & 7)
            ).unwrap());
            encoded.push(nes_num_to_char(
                ((addr >> 4) & 8) | ((val >> 4) & 7)
            ).unwrap());
            encoded.push(nes_num_to_char(
                (addr >> 4) & 7
            ).unwrap());
            encoded.push(nes_num_to_char(
                (addr & 8) | ((addr >> 12) & 7)
            ).unwrap());
            encoded.push(nes_num_to_char(
                ((addr >> 8) & 8) | (addr & 7)
            ).unwrap());
            encoded.push(nes_num_to_char(
                (val & 8) | ((addr >> 8) & 7)
            ).unwrap());
        },
        GGCode::NESCondCode { addr, cond, val } => {
            let addr: u32 = addr as u32;
            let cond: u32 = cond as u32;
            let val: u32 = val as u32;
            encoded.push(nes_num_to_char(
                ((val >> 4) & 8) | (val & 7)
            ).unwrap());
            encoded.push(nes_num_to_char(
                ((addr >> 4) & 8) | ((val >> 4) & 7)
            ).unwrap());
            encoded.push(nes_num_to_char(
                (addr >> 4) & 7
            ).unwrap());
            encoded.push(nes_num_to_char(
                (addr & 8) | ((addr >> 12) & 7)
            ).unwrap());
            encoded.push(nes_num_to_char(
                ((addr >> 8) & 8) | (addr & 7)
            ).unwrap());
            encoded.push(nes_num_to_char(
                (cond & 8) | ((addr >> 8) & 7)
            ).unwrap());
            encoded.push(nes_num_to_char(
                ((cond >> 4) & 8) | (cond & 7)
            ).unwrap());
            encoded.push(nes_num_to_char(
                (val & 8) | ((cond >> 4) & 7)
            ).unwrap());

        }
    };

    encoded
}
#[test]
fn test_byte_char_conversion() {
    for (b, c) in NES_CHARS.chars().enumerate() {
        assert_eq!(nes_char_to_num(c).unwrap(), b as u32);
        assert_eq!(nes_num_to_char(b as u32).unwrap(), c);
    }
}

#[test]
fn test_decode_nes() {
    assert_eq!(
        decode_nes("GOSSIP").unwrap(),
        GGCode::NESCode { addr: 0xD1DD, val: 0x14 }
    );
    assert_eq!(
        decode_nes("ZEXPYGLA").unwrap(),
        GGCode::NESCondCode { addr: 0x94A7, cond: 0x03, val: 0x02 }
    );
}

#[test]
fn test_encode() {
    // NES Codes
    assert_eq!(
        encode(GGCode::NESCode { addr: 0xD1DD, val: 0x14 }),
                String::from("GOISIP")
    );
    assert_eq!(
        encode(GGCode::NESCondCode { addr: 0x94A7, cond: 0x03, val: 0x02 }),
                String::from("ZEZPYGLA")
    );
}

#[test]
fn test_nes_alternate() {
    assert_eq!(
        String::from("GOSSIP"), get_alternate_code_nes("GOISIP").unwrap()
    );
    assert_eq!(
        String::from("GOISIP"), get_alternate_code_nes("GOSSIP").unwrap()
    );
    assert_eq!(
        String::from("ZEXPYGLA"), get_alternate_code_nes("ZEZPYGLA").unwrap()
    );
    assert_eq!(
        String::from("ZEZPYGLA"), get_alternate_code_nes("ZEXPYGLA").unwrap()
    );
}
