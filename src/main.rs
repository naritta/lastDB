
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use std::collections::HashMap;
use std::fs;
use std::net::TcpListener;

use std::io::{self, Read, Write, stdout};

fn load_yaml(path: &str) -> Vec<yaml_rust::Yaml> {
    let f = fs::read_to_string(path);
    let s = f.unwrap().to_string();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    docs
}

fn local_run() {
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
}

fn server_run() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
    }
}

fn main() {
    let path = "./src/main.yml";
    let docs = load_yaml(&path);
    let doc = &docs[0];

    println!("MAX memory size:{}", doc["setting"]["max_memory"].as_i64().unwrap());

    match doc["setting"]["mode"].as_str() {
        Some("local") => {
            local_run();
        },
        Some("server") => {
            server_run();
        },
        Some(key) =>println!("unreviewed."),
        None => {
            local_run();
        }
    }

    // let r = io::stdin();
    // let mut reader = r.lock();
    //
    // let w = io::stdout();
    // let mut writer = w.lock();

    // getValue(&mut reader, &mut writer);
}
