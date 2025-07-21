pub fn main(){
    println!("========= Variables =========");

    // Variable inmutable
    let nombre: &str = "Victor"; // cadena de texto estatica (tamaño fijo en memoria)
    println!("Mi nombre es: {nombre}");

    // Variable mutable
    let mut edad: u8 = 31; // entero sin signo de 8 bits
    println!("Mi edad ahora es de {edad} años");

    edad = 32;
    println!("Pero en 2025 cumpliré {edad} años");

    // Tipo de dato String
    let apellido: String = String::from("Saldivia"); // cadena de texto dinámica (tamaño en memoria cambia en tiempo de ejecución)
    println!("Mi apellido es: {apellido}\n");

    // Explicando la Interporlación de Valores
    println!("========= INTERPOLACIÓN =========");
    let mut numero1: i32 = 10; // entero de 32 bits
    numero1 = numero1 + 10;
    println!("{numero1}"); // se imprime el valor actual de numero1
    println!("{}", numero1 -1); // en el momento que se este ejecutando se resta 1 (pero no cambia su valor original)
    println!("{numero1}"); // sigue valiendo 20


}