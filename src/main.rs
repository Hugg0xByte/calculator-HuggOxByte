use calculator_huggoxbyte::calc1::{add, sub};
use calculator_huggoxbyte::calc2::{multiply, rate};

fn main() {
    println!("\n----- Testando a Biblioteca calculadora -----");

    let c = add(1, 2);
    println!("Resultado da soma de 1 + 2 : {}", c);

    let d = sub(5, 2);
    println!("Resultado da subtração de 1 - 2: {}", d);

    let e = multiply(6, 2);
    println!("Resultado da multiplicação de 1 * 2: {}", e);

    let f = rate(30, 4);
    println!("Resultado da divisão de 1 / 2: {}", f);

    println!("\n----- Fim dos testes manuais -----");
}
