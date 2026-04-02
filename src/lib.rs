pub fn gf_add(a: u8, b: u8) -> u8 {
    a ^ b
}

pub fn xtime(a: u8) -> u8 {
    let mask: u8 = 1u8 << 7;

    if a & mask != 0 {
        (a << 1) ^ 0x1b
    } else {
        a << 1
    }
}

pub fn gf_mul(a: u8, b: u8) -> u8 {
    let mut result: u8 = 0x00;
    let mut current_power: u8 = a; 

    for n in 0..8 {
        if b & (1u8 << n) != 0 {
            result ^= current_power;
            current_power = xtime(current_power);
        } else {
            current_power = xtime(current_power);
        }
    }

    result
}

pub fn gf_div(mut dividend: u16, divisor:u16) -> (u8, u16) {
    let mut deg_dividend: u32 = if dividend == 0 { 0 } else { 15 - dividend.leading_zeros() };
    let deg_divisor: u32 = 15 - divisor.leading_zeros();
    let mut quotient:u8 = 0x00;

    while deg_dividend >= deg_divisor {
        let shift: u32 = deg_dividend - deg_divisor;
        dividend ^= divisor << shift;
        quotient |= 1 << shift;
        if dividend == 0x00 {
            break;
        } else {
            deg_dividend = 15 - dividend.leading_zeros();
        }
    }

    (quotient, dividend)
}

pub fn gf_inv(a: u16) -> u8 {
    let p: u16 = 0x11b;
    let (mut r, mut newr) = (p, a);
    let (mut t, mut newt) = (0x00, 0x01);

    while newr != 0 {
        let (quotient, remainder) = gf_div(r as u16, newr as u16);
        (r, newr) = (newr, remainder);
        (t, newt) = (newt, t as u16 ^ gf_mul(quotient as u8, newt as u8) as u16);
    }

    t as u8
}

pub fn sbox(a: u8) -> u8 {
    let c: u8 = 0x63;
    let mut b_tild: u8 = 0x00;
    let mut b_tild_shifted:u8 = 0x00;

    if a != 0x00 {
        b_tild = gf_inv(a as u16)
    }

    for i in 0..4 {
        b_tild_shifted ^= b_tild.rotate_right(i + 4)
    }

    let b_dash = b_tild ^ b_tild_shifted ^ c;
    b_dash
}

pub fn sub_bytes(state: &mut [u8; 16]) {
    for i in 0..state.len() {
        state[i] = sbox(state[i])
    }
}

pub fn shift_rows(state: &mut [u8; 16]) {
    let map: [usize; 16] = [
        0, 1, 2, 3,
        5, 6, 7, 4,
        10, 11, 8, 9,
        15, 12, 13, 14
    ];
    let mut new_state: [u8; 16] = [0; 16];

    for i in 0..16 {
        new_state[i] = state[map[i]]
    }
    
    *state = new_state;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gf_add() {
        let result = gf_add(2, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_xtime() {
        let result = xtime(0x01);
        assert_eq!(result, 0x02);
    }

    #[test]
    fn test_gf_mul() {
        let result = gf_mul(0x13, 0x57);
        assert_eq!(result, 0xfe);
        println!("{}", gf_mul(0x05, 0x53));
    }

    //#[test]
    //fn test_gf_div() {
    //    let result = gf_div(0xfe, 0x13);
    //    assert_eq!(result, 0x57);
    //}
    
    #[test]
    fn test_gf_inv() {
        let result = gf_inv(0x53);
        assert_eq!(result, 0xca);
    }

    #[test]
    fn test_sbox() {
        let result = sbox(0x53);
        assert_eq!(result, 0xed);
    }

    #[test]
    fn test_sub_bytes() {
        let mut state: [u8; 16] = [
          0x19, 0x3d, 0xe3, 0xbe,
          0xa0, 0xf4, 0xe2, 0x2b,
          0x9a, 0xc6, 0x8d, 0x2a,
          0xe9, 0xf8, 0x48, 0x08,
        ];
        sub_bytes(&mut state);
        assert_eq!(state, [
          0xd4, 0x27, 0x11, 0xae,
          0xe0, 0xbf, 0x98, 0xf1,
          0xb8, 0xb4, 0x5d, 0xe5,
          0x1e, 0x41, 0x52, 0x30,
        ]);
    }

    #[test]
    fn test_shift_rows() {
        let mut state: [u8; 16] = [
          0xd4, 0xe0, 0xb8, 0x1e,
          0x27, 0xbf, 0xb4, 0x41,
          0x11, 0x98, 0x5d, 0x52,
          0xae, 0xf1, 0xe5, 0x30
         ];
        shift_rows(&mut state);
        assert_eq!(state, [
          0xd4, 0xe0, 0xb8, 0x1e,
          0xbf, 0xb4, 0x41, 0x27,
          0x5d, 0x52, 0x11, 0x98,
          0x30, 0xae, 0xf1, 0xe5,
        ]);
    }
}
