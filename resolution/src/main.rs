fn main() {
    let name: &str = "João Cícero"; // Variável imutável
    let mut age = 31;
    age += 1;

    println!("Hello, world!");
    println!("Hello {}!", name);
    println!("Years {}!", age);
}
