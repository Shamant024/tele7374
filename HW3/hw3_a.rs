fn main() {
    let mut w1: u16 = 0x2105;
    println!("w1 = 0x{:04x} [{:016b}]", w1, w1);
    w1 |= 1 << 2;
    println!("Set 3rd bit: w1 = 0x{:04x} [{:016b}]", w1, w1);

    let mut w2: u16 = 0x21ff;
    println!("w2 = 0x{:04x} [{:016b}]", w2, w2);
    w2 &= !(1 << 3);
    println!("Clear 4th bit: w2 = 0x{:04x} [{:016b}]", w2, w2);

    let mut w3: u16 = 0x2100;
    println!("w3 = 0x{:04x} [{:016b}]", w3, w3);
    w3 ^= 1 << 4;
    println!("Flip 5th bit: w3 = 0x{:04x} [{:016b}]", w3, w3);

    let mut w4: u8 = 0x80;
    println!("w4 = 0x{:02x} [{:08b}]", w4, w4);
    let mask1: u8 = (1 << 1) | (1 << 4);
    w4 |= mask1;
    println!("Set 2nd and 5th bit: w4 = 0x{:02x} [{:08b}]", w4, w4);

    let mut w5: u8 = 0xff;
    println!("w5 = 0x{:02x} [{:08b}]", w5, w5);
    let mask2: u8 = (1 << 0) | (1 << 6);
    w5 &= !mask2;
    println!("Clear 1st and 7th bit: w5 = 0x{:02x} [{:08b}]", w5, w5);

    let mut w6: u8 = 0x00;
    println!("w6 = 0x{:02x} [{:08b}]", w6, w6);
    let mask3: u8 = (1 << 2) | (1 << 3);
    w6 ^= mask3;
    println!("Flip 3rd and 4th bit: w6 = 0x{:02x} [{:08b}]", w6, w6);
}
