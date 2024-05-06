use std::borrow::Cow;
use std::env::args;
use std::fmt::Debug;
use std::io::stdin;
use std::io::{Read, Write};
use std::net;
use std::process::exit;
use std::time::{Duration, SystemTime};

use std::time::Instant;

pub fn gen_master_id() -> String {
    // for i in 0..100 {
    //     println!("{}:{}", i, char::from_u32(i).unwrap())
    // }

    // exit(0);
    let mut rnd = String::new();

    for _ in 0..40 {
        let seed = format!(
            "{:?}",
            SystemTime::elapsed(&SystemTime::UNIX_EPOCH).unwrap()
        );

        let mut v: u32 = 0;

        for b in seed.as_bytes() {
            v += *b as u32
        }

        let val = &format!("{:x}", v % 86 * 11)[0..1];

        rnd.push_str(val);
    }
    rnd
}

const ADDR: &'static str = "127.0.0.1:6379";

fn stream_helper(to_send: &str) -> Result<String, ()> {
    let mut stream = net::TcpStream::connect(ADDR).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    stream.write(to_send.as_bytes()).unwrap();
    stream.flush().unwrap();

    match stream.read_to_end(&mut buf) {
        Ok(_) => Ok(String::from_utf8(buf).unwrap()),
        Err(_) => Err(()),
    }
}

fn main() {
    // println!("{}", gen_master_id());
    // exit(0);

    let mut args = args();

    if args.len() < 2 {
        panic!("Not enough args, usage: <ip:port> <data>");
    }
    let _ = args.next();

    let address = args.next().unwrap();
    let mut stream = net::TcpStream::connect(&address).unwrap();

    stream
        .set_write_timeout(Some(Duration::from_secs(4)))
        .unwrap();

    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();

    println!("connected!");

    let mut stdin = stdin();

    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        let n = stdin.read(&mut buf).unwrap();
        let input_str = String::from_utf8(buf.to_vec()).unwrap();
        let repl = input_str.replace("\\r\\n", "\r\n");

        let a = stream.write(repl.as_bytes()).unwrap();
        stream.flush().unwrap();

        buf.fill(0);

        // stream.set_nonblocking(false).unwrap();
        // println!("wrote {} bytes", a);
        match stream.read(&mut buf) {
            Ok(n) if n > 0 => {
                println!("{}", String::from_utf8(buf.to_vec()).unwrap());
            }
            Ok(_) => (),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => (),
            Err(e) => {
                panic!("{}", e);
            }
        }

        // stream.set_nonblocking(true).unwrap();

        // println!("{}", String::from_utf8(buf.to_vec()).unwrap());
    }
}
