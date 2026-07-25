use std::io::stdin;

fn main() {
    loop {


        println!("Эта программа конвертирует градусы фаренгейта в градусы цельсия");
        println!("Напишите градусы в фаренгейтах (без знака)");

        let mut fahrenheit = String::new();

        stdin().read_line(&mut fahrenheit).expect("Failed to read line");
        let fahrenheit: f64 = fahrenheit.trim().parse().expect("Failed to parse fahrenheit");

        let  celsius = (fahrenheit - 32.0) / 1.8;

        println!("В фаренгейтах: {}, в цельсиях = {}", fahrenheit, celsius);

        println!("Вы хотите выйти? да/нет");

        let mut input = String::new();

        stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();

        if input.to_lowercase().as_str() == "да" {
            println!("Окиии");
            break;
        }

        else { continue; }

    }
}