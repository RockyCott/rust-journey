# Getting Started

we’ll discuss:

- Installing `Rust` on `Linux`, `macOS`, and `Windows`. (I use `Linux`)
- Writing a program that prints `Hello, world!`
- Using `cargo`, Rust’s package manager and build system

## Installing Rust

Para instalar Rust, lo más sencillo es usar [`rustup`](https://www.rust-lang.org/tools/install), una herramienta de línea de comandos que gestiona las versiones de Rust y sus herramientas asociadas. Es algo así como el `nvm` o el `volta` de Rust.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Updating and Uninstalling

```bash
rustup update
```

```bash
rustup self uninstall
```

---

## Hello, World

```bash
mkdir hello_world
cd hello_world
```

Next, make a new source file and call it `main.rs`. Rust files always end with the `.rs` extension.

```bash
touch main.rs
```

`main.rs`

```rust
fn main() {
    println!("Hello, world!");
}
```

Para compilar y ejecutar el programa, usa:

```bash
rustc main.rs
./main
```

Resultado:

```bash
Hello, world!
```

Rust es un lenguaje compilado. Eso significa que antes de poder ejecutar tu código, necesitas convertirlo a un programa binario usando el compilador (`rustc` en este caso).

```bash
rustc main.rs
```

Esto genera un archivo ejecutable que podemos correr directamente. En otros lenguajes como `JavaScript`, `Python` o `Ruby`, probablemente sea costumbre a que el código se ejecute con un solo comando (sin necesidad de compilar). En Rust, la compilación es un paso adicional, pero a cambio obtenemos un archivo ejecutable que se puede compartir y ejecutar en otras máquinas ¡incluso si no tienen Rust instalado!

---

## Using Cargo

`Cargo` is the Rust package manager and build system. It’s similar to `npm` for JavaScript.

### Creating a New Project

```bash
cargo new hello_cargo
cd hello_cargo
```

This creates a new directory called `hello_cargo` with the following structure:

```bash
hello_cargo
├── Cargo.toml
└── src
    └── main.rs
```

El archivo `Cargo.toml` es el equivalente al package.json de JavaScript. Contiene información del proyecto, como su nombre, versión y dependencias.

`Cargo.toml`

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

Cargo espera que el código fuente esté en la carpeta src. El resto del proyecto (como documentación o configuración) queda fuera de esa carpeta. Esta estructura hace que todo esté bien organizado desde el inicio.

### Building and Running the Project

To build and run the project:

```bash
cargo build
cargo run
Hello, world!
```

This will compile the project and create an executable file in the `target/debug` directory. The `cargo run` command will build the project if it hasn’t been built yet, and then run the executable.

La primera vez puede demorar un poco, pero después será más rápido porque Rust guarda los resultados intermedios.

```bash
cargo run
   Compiling hello_cargo v0.1.0 (/home/username/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello_cargo`
Hello, world!
```

También se puede verificar si el código compila sin generar un ejecutable, usando:

```bash
cargo check
    Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
        Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Esto es útil mientras se está desarrollando y se quiere obtener feedback rápido de errores de compilación sin esperar tanto como en cargo build.

Let’s recap what we’ve learned so far about Cargo:

- We can create a project using `cargo new`.
- We can build a project using `cargo build`.
- We can build and run a project in one step using `cargo run`.
- We can build a project without producing a binary to check for errors using `cargo check`.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the `target/debug` directory.

---

## Anatomy of a Rust Program

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn main()` define una función llamada main. Es el punto de entrada de cualquier programa en Rust.
- El cuerpo de la función está entre llaves `{}`.
- Dentro de `main`, usamos `println!()` para imprimir texto en pantalla. El `!` indica que `println` es una macro, no una función normal. (Las macros en Rust permiten hacer cosas que las funciones no pueden).
- El texto `"Hello, world!"` es el argumento que le pasamos a la macro.
- Cada línea termina en `;` (punto y coma).

---

## Building for Release

Cuando tu programa ya está listo para ser publicado o distribuido, puedes compilarlo con optimizaciones usando:

```bash
cargo build --release
```

Esto crea el ejecutable en target/release en lugar de target/debug. El binario será más rápido, pero tomará más tiempo en compilar. Por eso se recomienda usar el modo debug durante el desarrollo, y solo usar --release cuando ya está listo para producción o cuando necesitas medir su rendimiento real.
