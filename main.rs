use std::collections::HashMap;

// fn main() {
//     let mut kv = HashMap::new();
//
//     kv.insert(String::from("Blue"), 10);
//     kv.insert(String::from("Yellow"), 50);
//
//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }
//
//     let r = io::stdin();
//     let mut reader = r.lock();
//
//     let w = io::stdout();
//     let mut writer = w.lock();
//
//     writeln!(writer, "yes").unwrap();
//     //
//     // let _ = io::copy(&mut reader, &mut writer);
//     // writer.write("hello");
// }

use std::io::{self, Read, Write, stdout};

fn getValue(reader: &mut Read, writer: &mut Write) {
    let mut kv = HashMap::new();

    kv.insert(String::from("a"), String::from("Yellow"));
    kv.insert(String::from("b"), String::from("Blue"));

    const BUFFER_SIZE: usize = 32 * 1024;
    let mut buf = String::new();

    loop {
        let mut key = String::new();
        io::stdin()
            .read_line(&mut key)
            .expect("failed");
        print!("{}", key);
        let val = kv.get(&key).unwrap();
        writeln!(writer, "{}", val);
    }

    // while let Ok(n) = reader.read(&mut buf) {
    //     if n == 0 {
    //         break;
    //     }
    //     let key = String::from("a");
    //     let val = kv.get(&key).unwrap();
    //     writeln!(writer, "{}", val);
    //     // let _ = writer.write(val.as_bytes());
    // }
}

fn main() {
    let mut kv = HashMap::new();

    kv.insert(String::from("a"), String::from("Yellow"));
    kv.insert(String::from("b"), String::from("Blue"));

    const BUFFER_SIZE: usize = 32 * 1024;
    let mut buf = String::new();

    loop {
        let mut key = String::new();
        io::stdin()
            .read_line(&mut key)
            .expect("failed");
        print!("{}", key);
        let mut rawkey = key.trim_end();
        // let val = kv.get(&key);
        match kv.get(rawkey) {
            Some(review) => println!("{rawkey}: {review}"),
            None => println!("{rawkey} is unreviewed.")
        }
        // writeln!(writer, "{}", val);
    }
    // let r = io::stdin();
    // let mut reader = r.lock();
    //
    // let w = io::stdout();
    // let mut writer = w.lock();

    // getValue(&mut reader, &mut writer);
}
