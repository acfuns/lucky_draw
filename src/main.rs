use clap::Parser;
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(
    author = "acfuns <1haolong.sun@gmail.com>",
    version = "0.1.0",
    about = "Lucky Draw"
)]
struct Args {
    /// The number of lucky draw people
    #[clap(short, long, value_parser, default_value_t = 1)]
    num: u8,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut rng = rand::thread_rng();
    let mut users = Vec::new();
    let file = File::open("user_name.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let name = line?.trim().to_string();
        users.push(name);
    }
    println!("{:?}", users);
    for i in 0..args.num {
        println!("第 {} 次抽奖开始 ------------------------", i);
        let rand_idx: usize = rng.gen_range(0..users.len());
        let lucky_star = users.remove(rand_idx);
        sleep(Duration::from_secs(3));
        println!("恭喜昵称为 {} 的用户中奖了!", lucky_star);
    }
    println!("抽奖结束");
    Ok(())
}
