#![allow(dead_code)]

mod arrary;

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
    panic!("crash and burn");
}
