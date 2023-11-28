#[allow(unused)]
use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{IpAddr, Ipv4Addr,SocketAddr,  TcpListener, TcpStream},
    process,
};

pub fn hello() {
    println!("Hello!");
}

/*
pub fn init_server_on_localhost() -> Result<TcpListener, &'static str> {
    let ip_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = RUST_PORT;
    let socket_address = SocketAddr::new(ip_v4, port);

    assert_eq!(socket_address.port(), port);
    assert_eq!(socket_address.is_ipv4(), true);
    assert_eq!(socket_address.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    let listener = TcpListener::bind(socket_address).unwrap();
    if listener.local_addr().unwrap() != socket_address {
        return Err("Fail")
    }

    Ok(listener)
}
pub fn init_server_on_lan() -> Result<TcpListener, &'static str> {
    let ip_v4 = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 178));
    let port = RUST_PORT;
    let socket_address = SocketAddr::new(ip_v4, port);

    assert_eq!(socket_address.port(), port);
    assert_eq!(socket_address.is_ipv4(), true);
    assert_eq!(socket_address.ip(), ip_v4);

    let listener = TcpListener::bind(socket_address).unwrap();
    if listener.local_addr().unwrap() != socket_address {
        return Err("Fail")
    }

    Ok(listener)
}

*/
