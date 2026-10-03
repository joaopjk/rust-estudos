use std::io;

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
    } else {
        println!("{} is less than {}", number1, number2);
    }

    // ‘Input’ de dados
    let mut number3 = String::new();
    io::stdin()
        .read_line(&mut number3)
        .expect("Failed to read line");

    let mut number4 = String::new();
    io::stdin()
        .read_line(&mut number4)
        .expect("Failed to read line");

    if convert_to_int(&number3) > convert_to_int(&number4) {
        println!("{} is greater than {}", number3, number4);
    } else {
        println!("{} is less or equal than {}", number4, number3);
    }

    // While
    let mut soma = 0;
    let mut valor_entrada = String::new();

    io::stdin()
        .read_line(&mut valor_entrada)
        .expect("Failed to read line");
    let mut valor_i32 = convert_to_int(&valor_entrada);

    while valor_i32 != 0 {
        let r = valor_i32 % 10;
        soma = soma + r;
        valor_i32 = valor_i32 / 10;
    }

    println!("O valor da soma dos dígitos é {}", soma);

    let mut entrada_fatorial = String::new();
    io::stdin()
        .read_line(&mut entrada_fatorial)
        .expect("Failed to read line");
    let mut fatorial = 1;
    let mut entrada_int = convert_to_int(&entrada_fatorial);

    while entrada_int > 1 {
        fatorial = fatorial * convert_to_int(&entrada_fatorial);
        entrada_int = entrada_int - 1;
    }
}

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}
