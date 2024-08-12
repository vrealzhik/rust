use std::assert_eq;
use std::net::IpAddr;

fn main() {
    let ip_str: &str = "192.168.50.1";
    let bin_str = "11000000101010000011001000000001";
    let ip: IpAddr = ip_str.parse().expect("Некорректный ip");
    ip_formatter(ip, &bin_str);
}

fn ip_formatter(ip: IpAddr, bin_str: &str) {
    match ip {
        IpAddr::V4(v4) => {
            let bin = v4.octets();
            let vec: Vec<String> = bin.iter().map(|&octet| format!("{:08b}", octet)).collect();
            let str = vec.join("");
            println!("Двоичное представление v4 {}: {}", ip, vec.join(""));
            assert_eq!(str, bin_str)
        }
        _ => todo!(),
    }
}
