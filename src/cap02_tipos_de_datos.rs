pub fn main(){
 // Tipo de dato String
    println!("========= STRINGS =========");
    let pais: String = String::from("Chile"); // cadena de texto dinámica (tamaño en memoria cambia en tiempo de ejecución)
    println!("El país donde vivo es {pais}\n");

    // Números Enteros
    println!("========= INÚMEROS ENTEROS =========");
    let mut edad: i32 = 31; // entero de 32 bits
    edad = edad + 10;
    println!("{edad}"); // se imprime el valor actual de edad (41)
    println!("{}", edad -1); // en el momento que se este ejecutando se resta 1 (pero no cambia su valor original)
    println!("{edad}"); // sigue valiendo 41

    // Número Flotantes
    println!("\n========= NÚMEROS FLOTANTES =========");
    let mut peso: f64 = 70.5;
    println!("Mi peso es de: {peso}");

    // peso = peso + 20; // error, como es un lenguaje de tipado extricto no se puede sumar un int y un float

    peso = 75.5;
    println!("Peso actual: {peso}");

    // Booleanos
    println!("\n========= BOOLEANOS =========");
    let mut booleano: bool = false;
    println!("{booleano}");

    booleano = true;
    println!("{booleano}");

    // Char
    println!("\n========= CHAR =========");
    let letra: char = 'A'; // un solo caracter
    println!("{letra}");


}