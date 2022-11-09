
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
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

fn getValue(reader: &mut dyn Read, writer: &mut dyn Write) {
    let s =
"
foo:
    - list1
    - list2
bar:
    - 1
    - 2.0
";
    let docs = YamlLoader::load_from_str(s).unwrap();

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
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed");
        let mut split_input = input.split_whitespace();
        // let mut type = split_input.next();
        // let mut input = split_input.next();
        // let mut rawkey = key.trim_end();
        match split_input.next() {
            Some("set") => match split_input.next() {
                Some(key) => match split_input.next() {
                    Some(val) => {
                        kv.insert(String::from(key), String::from(val));
                        println!("set {key}:{val}.");
                    },
                    None => println!("no value set.")
                },
                None => println!("unreviewed.")
            },
            Some("get") =>match split_input.next() {
                Some(key) => match kv.get(key) {
                    Some(val) => println!("got {val} for {key}"),
                    None => println!("no value found fpr {key}.")
                },
                None => println!("unreviewed.")
            },
            Some(key) =>println!("unreviewed."),
            None => println!("unreviewed.")
        }
        // println!("input: {input}.")
        // let val = kv.get(&key);
        // writeln!(writer, "{}", val);
    }
    // let r = io::stdin();
    // let mut reader = r.lock();
    //
    // let w = io::stdout();
    // let mut writer = w.lock();

    // getValue(&mut reader, &mut writer);
}
