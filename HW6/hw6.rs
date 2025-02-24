fn encrypt(s: String) -> String {
    let mut result = String::new();
    for c in s.chars() {
        let ascii = c as u32;
        if ascii >= 65 && ascii <= 90 {
            result.push((ascii + 30) as u8 as char);
        } else if ascii >= 97 && ascii <= 128 {
            result.push((ascii - 30) as u8 as char);
        } else {
            result.push(c);
        }
    }
    result
}

fn decrypt(s: String) -> String {
    let mut result = String::new();
    for c in s.chars() {
        let ascii = c as u32;
        if ascii >= 95 && ascii <= 120 {
            result.push((ascii - 30) as u8 as char);
        } else if ascii >= 67 && ascii <= 98 {
            result.push((ascii + 30) as u8 as char);
        } else {
            result.push(c);
        }
    }
    result
}

// Main using direct function calls
fn main_direct() {
    let original = String::from("This is a test");
    let encrypted = encrypt(original.clone());
    println!("Encrypted String is: {}", encrypted);
    let decrypted = decrypt(encrypted);
    println!("Decrypted String is: {}", decrypted);
    if original == decrypted {
        println!("Original and decrypted strings match!");
    } else {
        println!("Strings do not match!");
    }
}

// Main using sequential blocking threads
fn main() {
    let original = String::from("This is a test");
    let original_clone = original.clone(); 

    let encrypt_thread = std::thread::spawn(move || {
        encrypt(original_clone)  
    });
    let encrypted = encrypt_thread.join().unwrap();
    println!("Encrypted String is: {}", encrypted);

    let decrypt_thread = std::thread::spawn(move || {
        decrypt(encrypted)
    });
    let decrypted = decrypt_thread.join().unwrap();
    println!("Decrypted String is: {}", decrypted);

    if original == decrypted {  
        println!("Original and decrypted strings match!");
    } else {
        println!("Strings do not match!");
    }
}
