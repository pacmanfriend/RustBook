fn main() {
    println!("Hello, world!");
    vars();
    print_const();
    func_with_params(150);
    test123();
}

fn vars() {
    let mut a: i32 = 5;
    println!("a is {}", a);

    a = 6;
    println!("a is {} ", a);
}

const A: i32 = 1024;

fn print_const() {
    println!("Const A is {}", A)
}

fn func_with_params(x: i32) {
    println!("Param x is {}", x);
}

fn test123() -> i32 {
    let x = 6;

    let y = {
        let x = 3;
        x + 1
    };

    let a = [2; 10];

    x + y
}
