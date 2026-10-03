fn main() {
    let name: &str = "João Cícero"; // Variável imutável
    let mut age = 31;
    age += 1;

    println!("Hello, world!");
    println!("Hello {}!", name);
    println!("Years {}!", age);

    // Tipos de Dados
    let x: i64 = 32;
    let y: u32 = 54;
    let f: f32 = 1.2;
    let b: bool = false;

    println!("{},{},{},{}", x, y, f, b);

    // Fluxo de controle
    let number1 = 24;
    let number2 = 42;

    if number1 > number2 {
        println!("{} is greater than {}", number1, number2);
    }
    else {
        println!("{} is less than {}", number1, number2);
    }
}
