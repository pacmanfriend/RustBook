use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    rectangle_area_calculator()
}

fn _game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Угадайте число!");

    loop {
        println!("Введите загаданное число:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Не удалось прочитать строку!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Введите число!");
                continue;
            }
        };


        println!("Вы ввели: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком малое число!"),
            Ordering::Greater => println!("Слишком большое число!"),
            Ordering::Equal => {
                println!("Вы выиграли!");
                break;
            }
        }
    }
}

struct Rectangle(i32, i32);

fn rectangle_area_calculator() {
    let rect = Rectangle(30, 50);

    let rect_area = area(rect);

    println!("Площадь прямоугольника равна: {rect_area}")
}

fn area(rect: Rectangle) -> i32 {
    rect.0 * rect.1
}