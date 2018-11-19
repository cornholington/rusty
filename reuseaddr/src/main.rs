extern crate net2;

use net2::UdpBuilder;

fn main() {
    println!("Hello, world!");

    let udp1 = UdpBuilder::new_v4().unwrap();
    udp1.reuse_address(true).unwrap();
    assert!(udp1.get_reuse_address().unwrap());

    let _sock1 = udp1.bind("0.0.0.0:8000").unwrap();

    let udp2 = UdpBuilder::new_v4().unwrap();
    udp2.reuse_address(true).unwrap();
    assert!(udp2.get_reuse_address().unwrap());

    let _sock2 = udp2.bind("0.0.0.0:8000").unwrap();
}
