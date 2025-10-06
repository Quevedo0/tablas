//TABLAS DE MULTIPLICAR--KEVIN SALAS--CIENCIAS DE DATOS 

fn main() {

    for i in 1..=10 {
        println!("tabla de multiplicar del {}: ", i);
        for j in 1..=10 {
            println!("{} ", i * j);
        }
        println!();
    }
}

