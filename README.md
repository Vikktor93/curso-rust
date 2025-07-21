## **Curso de Rust Básico**

Repositorio con ejemplos y ejercicios paso a paso para iniciarse en **Rust**: variables, tipos de datos, control de flujo, ciclos,módulos, etc.

### Tabla de contenidos

- [Descripción](#descripción)  
- [Requisitos](#requisitos)  
- [Instalación](#instalación)  
- [Estructura del proyecto](#estructura-del-proyecto)  
- [Uso](#uso)  
- [Capítulos](#capítulos)  
- [Licencia](#licencia)  

### Descripción

Este repositorio agrupa una serie de pequeños capítulos diseñados para que cualquier persona pueda iniciar en el lenguaje de programación de Rust. Primeros pasos:

1. Instalar y usar Rust desde cero.  
2. Comprender las bases del lenguaje: variables, tipos, control de flujo, etc. 
3. Aprender el sistema de módulos de Rust (módulos, `Cargo.toml`, binarios).  
4. Explorar conceptos clave: ownership, borrowing, estructuras, patrones, manejo de errores. 

Cada módulo es un archivo **`capXX_*.rs`** independiente con su propia función `main()` que ilustra el tema en cuestión. El archivo `main.rs` los importa y los ejecuta de forma secuencial.

### Requisitos

- **Rust** (versión 1.88.0 o superior)  
- **Cargo** (gestor de paquetes que viene con Rust)  
- Git  
- Editor de texto a elección (VS Code, Cursor, Windsurf)

Si no tienes Rust instalado, sigue la guía oficial:  
<https://www.rust-lang.org/tools/install>

### Instalación

```bash
# Clona el repositorio
git clone https://github.com/Vikktor93/curso-rust
cd curso-rust

# Compila dependencias
cargo build
```

### Estructura del Proyecto

```
├── src/
│   ├── main.rs
│   ├── cap01_variables.rs
│   ├── cap02_tipos_de_datos.rs
│   └── cap03_control_de_flujo.rs
├── .gitignore
├── Cargo.toml
├── Cargo.lock
└── README.md
```

### Uso
Para ejecutar todos los ejemplos de forma secuencial:

```bash
cargo run
```
Verás en consola la salida de cada módulo:

```
========= Rust Básico =========
-- Capítulo 01: Variables --
let mut edad: u8 = 31; // entero sin signo de 8 bits
println!("Mi edad ahora es de {edad} años");
...

-- Capítulo 02: Tipos de datos --
...
```

Si quieres enfocarte en un único capítulo:

- Comenta las llamadas en `main.rs` de los módulos que no quieras ejecutar.
- Vuelve a compilar y ejecutar con `cargo run`.


### Capítulos

| Archivo                          | Tema                                               |
| -------------------------------- | ---------------------------------------------------|
| `cap01_variables.rs`             | Variables, mutabilidad, interporlación             |
| `cap02_tipos_de_datos.rs`        | Enteros, flotantes, bool, strings, tuplas, arrays  |
| ...                              | ...                                                |


### Licencia
Este proyecto está bajo la Licencia MIT. Siéntete libre de usar, modificar y distribuir.