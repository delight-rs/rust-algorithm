#![allow(dead_code)]

mod arrary;
mod string;

trait IpAddr {
    fn diplay(&self);
}

struct V4(String);

impl IpAddr for V4 {
    fn diplay(&self) {
        println!("ipv4: {:?}", self.0);
    }
}

struct V6(String);

impl IpAddr for V6 {
    fn diplay(&self) {
        println!("ipv6: {:?}", self.0);
    }
}

fn main() {
    let ip = V4("198.168.31.91".to_string());
    ip.diplay();
    // panic!("crash and burn");
}
