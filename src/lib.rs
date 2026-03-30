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
        eprintln!("{}, {}, {}", dividend, quotient, divisor);
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
        //eprintln!("{} {} {}", r, newr, quotient);
    }

    t as u8
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
}
