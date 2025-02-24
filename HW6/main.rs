extern "C" {
    fn mul4(x: u32) -> u32;
}

fn main() {
    for i in 0..=10 {
        let result = unsafe { mul4(i) };
        println!("{}x4 = {}", i, result);
    }
}
