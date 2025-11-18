use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数を当てよう！");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("予想を入力してね");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("行の読み込みに失敗");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("有効な数字を入力してください！");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎます！"),
            Ordering::Greater => println!("大きすぎます！"),
            Ordering::Equal => {
                println!("正解です！");
                break;
            },
        }
    }
}
