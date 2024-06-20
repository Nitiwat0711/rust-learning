fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    fn route(_ip_kind: IpAddrKind) {}

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    enum IpAddrKind {
        V4,
        V6,
    }

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // using enums
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    // Enhanced Enums
    let home = IpAddr::V4(String::from(127, 0, 0, 1));
    let loopback = IpAddr::V6(String::from("::1"));

    // Using structs

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
}
