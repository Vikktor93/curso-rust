pub fn main(){
    println!("========= CONTROL DE FLUJO =========");

    let edad: i32 = 31;

    if edad >= 0 && edad < 18{
        println!("La persona es menor de edad");
    } else if edad >= 18 && edad < 65{
        println!("La persona es un adulto");
    } else if edad >= 65 && edad <= 130{
        println!("La persona es un adulto mayor");
    } else {
        println!("La edad ingresa no es valida");
    }

}