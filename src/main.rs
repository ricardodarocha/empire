#[derive(std::fmt::Debug)]
enum Aldeao {
    None,
    Food(u8),
    Wood(u8),
    Gold(u8),
    Stone(u8),
}

fn main() {
    let aldeao = Aldeao::None;
    println!("Hello, {:?}!", aldeao);
    let aldeao = Aldeao::Food(1);
    println!("Hello, {:?}!", aldeao);
    let aldeao = Aldeao::Wood(1);
    println!("Hello, {:?}!", aldeao);
    let aldeao = Aldeao::Gold(1);
    println!("Hello, {:?}!", aldeao);
    let aldeao = Aldeao::Stone(1);
    println!("Hello, {:?}!", aldeao);
}
