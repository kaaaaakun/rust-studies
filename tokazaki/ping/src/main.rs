use std::process;
use std::{thread, time::Duration};
use socket2::{Socket, Domain, Type, Protocol};
use icmp;
use std::net::{IpAddr, Ipv4Addr};



fn is_ipv4_address(input: &str) -> bool {
    input.parse::<Ipv4Addr>().is_ok()
}

// 引数を受け取る
// pingを打てる形のurlに変換
// 1秒に一回そのコマンドを実行
// その時の通信状況を表示
// ctl+Cを押したときにそれまでの結果が出るように

fn validate_args(args: &[String]) -> Result<&str, &str> {
    match args.len() {
        1 => Err("not enough arguments"),
        2 => {
            if is_ipv4_address(&args[1]) {
                Ok(&args[1])
            } else {
                Err("invalid ipv4 address")
            }
        }
        _ => Err("too many arguments"),
    }
}

fn print_ping_status(ip_address: &str, count: i32) {
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let ping_result = icmp::IcmpSocket::connect(localhost_v4);

    match ping_result {
        Ok(_ping) => {
            // IPアドレスは引数として渡されている ip_address を使用する
            dbg!("Destination IP: {}, Count: {}", ip_address, count); // ip_address をそのまま出力
        },
        Err(err) => {
            eprintln!("Error connecting to ICMP socket: {}", err);
        }
    }


    // TODO: ここで実際の通信を行う
    //let status = if count % 2 == 0 {
    //    "64 bytes from"
    //} else {
    //    "Request timeout for"
    //};
    //println!(
    //    "{} {}: icmp_seq={} ttl=64 time=0.0 ms",
    //    status, ip_address, count
    //);
}

fn main() {
    let mut count: i32 = 0;

    // 引数のバリデーション
    let args: Vec<String> = std::env::args().collect();
    let ip_address = match validate_args(&args) {
        Ok(ip) => {
            println!("Valid IPv4 address: {}", ip);
            ip.to_string()
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    //ctl+C関連
    let ip_addr_clone = ip_address.clone();
    ctrlc::set_handler(move || {
        // TODO: データを保存しておいて、ここで表示する
        println!("");
        println!("--- {} ping statistics ---", ip_addr_clone);
        println!("{} packets transmitted, {} packets received, 0.0% packet loss", count, count);
        println!("round-trip min/avg/max/stddev = 6.597/7.790/8.979/0.824 ms");
        // ここに結果を表示する処理を表示する
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // 最初のメッセージ
    println!("PING {} ({}): 56 data bytes", &ip_address, &ip_address);


    let socket = Socket::new(
        Domain::IPV4,
        Type::DGRAM,
        Some(Protocol::ICMPV4)
    ).expect("Failed to create socket");
    dbg!(&socket);

    // soket関連
    loop {
        print_ping_status(&ip_address, count);
        thread::sleep(Duration::from_secs(1));
        count += 1;
    }
}
