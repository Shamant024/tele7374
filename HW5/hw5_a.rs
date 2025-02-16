use std::thread;
use std::time::Duration;

fn main() {
    let handle1 = thread::spawn(move || {
        let handle2 = thread::spawn(move || {
            let handle3 = thread::spawn(move || {
                for i in 0..10 {
                    println!("T3 {}", i);
                    thread::sleep(Duration::from_secs(1));
                }
            });
            for i in 0..10 {
                println!("T2 {}", i);
                thread::sleep(Duration::from_secs(1));
            }
            handle3.join().unwrap();
        });
        for i in 0..10 {
            println!("T1 {}", i);
            thread::sleep(Duration::from_secs(1));
        }
        handle2.join().unwrap();
    });

    handle1.join().unwrap();
    std::process::exit(0);
}
