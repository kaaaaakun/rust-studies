use std::net::Ipv4Addr;

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

fn main() {
    //ctl+C関連

    // 引数のバリデーション
    let args: Vec<String> = std::env::args().collect();
    let ip_address = match validate_args(&args) {
        Ok(ip) => {
            println!("Valid IPv4 address: {}", ip);
            ip
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // soket関連
    println!("-- {ip_address} --");
}
