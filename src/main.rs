use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("数字を予想してね!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop { 
        println!("あなたの予想した数字を入力");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗しました。");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        
        println!("あなたの予想：{}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎです。"),       
            Ordering::Greater => println!("大きすぎです。"),      
            Ordering::Equal => {
                println!("おめでとう。予想的中です。");
                break;
            }
        }
    }
}