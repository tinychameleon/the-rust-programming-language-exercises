#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// To store the actual IP Address data you could use a struct.
// But an enum does this better with variants using data.
//#[derive(Debug)]
//struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
//}

#[derive(Debug)]
enum IpAddr {
    // Enum variants can also store different data types.
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing {:?}", ip_kind);
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    //let home = IpAddr {
    //    kind: IpAddrKind::V4,
    //    address: String::from("127.0.0.1"),
    //};

    //let loopback = IpAddr {
    //    kind: IpAddrKind::V6,
    //    address: String::from("::1"),
    //};

    //let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);
}
