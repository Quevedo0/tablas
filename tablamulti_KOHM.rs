fn main() {
    for i in 1..=10 {
        println!("Tabla de multiplicación de {}",i);
        for j in 1..=10 {
        println!("{} x {} = {}", i, j, i * j);
        }
        println!();
    }
}