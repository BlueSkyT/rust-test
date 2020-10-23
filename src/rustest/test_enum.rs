#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
    V8,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

pub fn test_enum() {
    let ipaddr = IpAddr {
        kind: IpAddrKind::V8,
        addr: String::from("127.0.0.1"),
    };
    println!("{:?}", ipaddr);

    match ipaddr.kind {
        IpAddrKind::V4 => {
            println!("is ip v4");
        },
        IpAddrKind::V6 => {
            println!("is ip v6")
        },
        _ => {
            println!("_");
        }
    }
}

pub fn test_match() {

}