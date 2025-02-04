fn swap_by_value(x: u8) -> u8 {
    // Extract 2nd and 4th bits
    let mask2 = (x >> 1) & 0x01;
    let mask4 = (x >> 3) & 0x01;
    
    // Clear 2nd and 4th bits
    let mut result = x & !(1 << 1);
    result &= !(1 << 3);
    
    // Set bits in swapped positions
    result |= mask2 << 3;
    result |= mask4 << 1;
    
    result
}

fn swap_by_ref(x: &mut u8) {
    // Extract 2nd and 4th bits
    let mask2 = (*x >> 1) & 0x01;
    let mask4 = (*x >> 3) & 0x01;
    
    // Clear 2nd and 4th bits
    *x &= !(1 << 1);
    *x &= !(1 << 3);
    
    // Set bits in swapped positions
    *x |= mask2 << 3;
    *x |= mask4 << 1;
}

fn main() {
    println!("Word\tBinary\t\t\tBy Value\t\tBy Reference");
    
    for i in 0..=255 {
        let y = i as u8;
        let swapped_by_val = swap_by_value(y);
        
        let mut ref_y = y;
        swap_by_ref(&mut ref_y);
        
        println!(
            "0x{:02x}\t{:08b}\t\t{:08b}\t\t{:08b}",y, y, swapped_by_val, ref_y);
    }
    
    std::process::exit(0);
}
