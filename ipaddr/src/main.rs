fn main() {
    //    let ip: std::net::Ipv4Addr = [0, 0, 0, 0]);
    println!("{:?}", std::net::Ipv4Addr::from(0));
    println!(
        "{:?}",
        std::net::SocketAddr::from((std::net::Ipv4Addr::from(0), 0))
    );
    println!("{:?}", std::net::SocketAddr::from(([0, 0, 0, 0], 0)));
    println!("{:?}", std::net::SocketAddr::from(([0; 4], 0)));

    println!("{:?}", std::net::SocketAddr::from(([0; 8], 0)));
    println!("{:?}", std::net::SocketAddr::from(([0; 16], 0)));

    println!("{:?}", std::net::SocketAddr::from(("0.0.0.0", 0)));

    let a: std::net::SocketAddr = ([0; 16], 0);
}
