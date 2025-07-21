pub fn main(){
    println!("========= Variables =========");

    // Variable inmutable
    let nombre: &str = "Victor";
    println!("Mi nombre es: {nombre}");

    // Variable mutable
    let mut edad: i8 = 31; // entero de 8 bits
    println!("Mi edad ahora es de {edad} años");

    edad = 32;
    println!("Pero en 2025 cumpliré {edad} años");
}