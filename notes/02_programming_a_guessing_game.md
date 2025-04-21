# Programming a Guessing Game

Se aprende sobre let, match, métodos, funciones asociadas, crates externos ¡y más!

## ¿De qué trata este juego?

The program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.

```bash
cargo new guessing_game
cd guessing_game
```

---

## Processing a Guess

Vamos a modificar `src/main.rs` para empezar el juego. Queremos pedirle al usuario que escriba un número, capturar ese número y luego imprimirlo en pantalla.

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

### Importando librerías estándar

```rust
use std::io;
```

Esta línea importa el módulo io de la biblioteca estándar (std), que es donde vive la funcionalidad para entrada/salida (como leer del teclado).

Aunque Rust incluye algunas funciones por defecto a través de un conjunto llamado prelude, no todas las funcionalidades están disponibles automáticamente. En este caso, como io no está en el prelude, debemos traerlo explícitamente con use.

La palabra clave `use` es una forma de importar módulos y funciones de otros módulos. En este caso, estamos importando el módulo io de la biblioteca estándar. Podemos decir que es equivalente al `import` de JavaScript o Python.

keyword allows you to bring a module into scope. The `std::io` module contains the `stdin` function, which is used to read input from the standard input stream (usually the keyboard). The `read_line` method reads a line of input from the user and stores it in the variable `guess`.

El operador `::` es el operador de ruta de Rust. Se usa para acceder a elementos dentro de módulos o estructuras. En este caso, `std::io` es el módulo y `stdin` es una función dentro de ese módulo. Podemos pensar en `::` como el equivalente a un punto (.) en otros lenguajes de programación, pero cuidado, no es lo mismo. En Rust, `::` se usa para acceder a funciones y tipos dentro de módulos, mientras que `.` se usa para acceder a métodos y campos de estructuras o instancias de tipos. Luego veremos más sobre esto.

### Storing Values with Variables

```rust
let mut guess = String::new();
```

Aquí estamos declarando una variable llamada guess. Observa varios elementos:

- `let` es la palabra clave para declarar variables.
- `mut` indica que la variable es mutable, es decir, su valor puede cambiar.
- `String::new()` crea una nueva cadena vacía. [`String`](https://doc.rust-lang.org/std/string/struct.String.html) es un tipo dinámico, capaz de crecer según el contenido.
- La sintaxis `::new()` es típica de funciones asociadas a tipos.

Por defecto, en Rust todas las variables son inmutables, lo cual ayuda a evitar errores inesperados. Pero en este caso, necesitamos mutabilidad porque vamos a modificar el contenido de guess luego.

We use the let statement to create the variable. Here’s another example:

```rust
let apples = 5;
```

To make a variable mutable, we add mut before the variable name:

```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

### Receiving User Input

Una de las líneas más interesantes (y densas) del código que escribimos es esta:

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

Aunque parece simple, esconde muchos detalles importantes. Vamos a analizarla paso a paso:

#### 1. ¿Qué es io::stdin()?

Rust tiene una biblioteca estándar (la `std`) que incluye muchas utilidades. Una de ellas es el módulo `io` (de input/output), que contiene funciones para interactuar con la entrada y salida del sistema.

Cuando escribimos:

```rust
io::stdin()
```

Estamos invocando la función `stdin()` del módulo `io`. Esta función nos devuelve un manejador de entrada estándar, es decir, algo que representa el teclado del usuario. En realidad, lo que devuelve es una instancia del tipo:

```rust
std::io::Stdin
```

Este tipo encapsula la entrada estándar de la terminal. Si no hubiéramos escrito la línea `use std::io;` al principio del archivo, entonces tendríamos que escribir esta llamada así, de forma completa:

```rust
std::io::stdin()
```

Rust permite acortar los nombres de los módulos usando la instrucción `use`, que es muy útil para evitar repeticiones y escribir código más limpio.

#### 2. ¿Qué hace `.read_line(&mut guess)`?

Ahora que tenemos el manejador del teclado, queremos decirle: "lee lo que el usuario escriba". Para eso, llamamos el método `read_line` sobre el resultado de `stdin()`:

```rust
.read_line(&mut guess)
```

Este método espera una referencia mutable a una variable de tipo `String`, donde va a guardar lo que el usuario escriba.

Pero aquí vienen varias cosas clave:

`&mut guess`

- `&` significa que estamos pasando una referencia, en lugar de mover el valor. Esto es eficiente: evitamos copiar la variable entera.
- `mut` significa que esa referencia permite modificar el contenido de `guess`.

Es decir, estamos diciendo: "Toma acceso mutable a esta variable y modifícala directamente dentro de `read_line`".

En Rust, las referencias son inmutables por defecto. Si quisiéramos pasar una referencia inmutable, sería `&guess`. Pero eso no nos serviría en este caso, porque queremos escribir en la variable, no solo leerla. Por eso necesitamos `&mut guess`.

#### 3. ¿Por qué `String` y no `&str`?

La función `read_line` necesita algo que pueda crecer dinámicamente, porque no sabemos cuántos caracteres escribirá el usuario. El tipo `String` es una cadena mutable y dinámica. En cambio, el tipo `&str` (una string slice) es inmutable y generalmente se usa para textos literales (`"Hola"`).

Además, `read_line` no reemplaza el contenido de la variable: lo agrega al final. Por eso es importante inicializar guess con `String::new()` al principio.

#### 4. ¿`read_line` retorna algo?

`read_line` devuelve un valor del tipo:

```rust
Result<usize, std::io::Error>
```

Esto es un `enum` (tipo enumerado) que indica si la operación fue exitosa o si ocurrió un error.

- Si todo salió bien, devuelve `Ok(n)`, donde `n` es el número de bytes leídos.
- Si algo falló, devuelve `Err(e)`, donde `e` es el error ocurrido.

Rust nos "obliga" a manejar ese `Result`, ya sea usando `match`, `unwrap`, `expect`, o algún otro mecanismo. En nuestro caso, usamos:

```rust
.expect("Failed to read line");
```

Esto dice: "Si todo salió bien, sigue normal. Si hubo un error, termina el programa y muestra este mensaje."

Esto es útil durante el desarrollo, porque nos permite detectar errores sin complicar demasiado el flujo del programa.

La llamada completa está escrita en varias líneas por legibilidad, pero podríamos haberlo escrito así:

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

#### ¿Por qué es tan importante esta línea?

En una sola expresión, estamos viendo varios pilares de Rust:

- Cómo usar módulos y funciones asociadas.
- Cómo trabajar con referencias y mutabilidad.
- Cómo se maneja la entrada del usuario.
- Cómo Rust promueve el manejo seguro de errores usando Result.

---

### Imprimir en consola con println

Después de obtener la entrada del usuario con read_line, generalmente queremos hacer algo con ese valor. Por ahora, simplemente vamos a imprimirlo para asegurarnos de que todo funcione correctamente:

```rust
println!("You guessed: {}", guess);
```

o:

```rust
println!("You guessed: {guess}");
```

#### ¿Qué está pasando aquí?

Rust usa el macro `println!` para imprimir texto en la consola. Es muy poderoso y flexible, y aquí lo estamos usando con una característica llamada interpolación de variables.

- Las llaves `{}` son marcadores de posición. Podemos imaginar que son como espacios reservados en una cadena de texto.
- Al escribir `{guess}`, Rust reemplaza eso por el contenido actual de la variable guess.

Este tipo de formato se llama interpolación de nombres. En Rust, es una forma de insertar valores dentro de cadenas de texto de manera segura y eficiente. Algo así como el template string de JavaScript, pero con un poco más de magia.

Por ejemplo:

```rust
let name = "Alice";
println!("Hello, {name}");
```

Imprime:

```plaintext
Hello, Alice
```

Otro ejemplo más complejo:

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

Imprime:

```plaintext
x = 5 and y + 2 = 12
```

Momento de ejecutar `cargo run` y probar el programa. Debería pedirte un número y luego imprimirlo en pantalla.

---

## Generating a Secret Number

Rust, por sí solo, no incluye funciones para generar números aleatorios en su biblioteca estándar (es decir, no trae eso "de fábrica"). Pero no hay de que preocuparse: la comunidad de Rust ha creado muchas herramientas que podemos usar, y una de ellas es el `crate` (paquete) llamado rand, que justamente nos permite trabajar con números aleatorios de manera sencilla. Así como en JavaScript se usa la biblioteca de depedencias [`npmjs`](https://www.npmjs.com/), en Rust usamos [`crates.io`](https://crates.io/) para encontrar y compartir crates. La mayoría de los crates son de código abierto y están disponibles para cualquier persona que quiera usarlos.

### What is a `crate`?

Un `crate` en Rust es básicamente un paquete de código que puede incluir funciones, estructuras y otros recursos que podemos reutilizar. Hay dos tipos principales:

- Crates binarios: son ejecutables, como el programa que estás creando.
- Crates de biblioteca: son conjuntos de código pensados para ser usados por otros programas, pero no se ejecutan por sí solos.

El crate `rand` es una biblioteca. No lo vamos a ejecutar directamente, pero sí vamos a usar su funcionalidad dentro del proyecto.

Para agregar un crate a nuestro proyecto, hay dos formas:

1. Editar el archivo `Cargo.toml` y agregar la dependencia manualmente.

    ```toml
    [dependencies]
    rand = "0.9.1"
    ```

2. Usar el comando `cargo add` para agregar la dependencia automáticamente.

    ```bash
    cargo add rand
    ```

### Probando que todo funcione

Una vez agregada la dependencia en Cargo.toml, ejecuta este comando:

```bash
cargo build
```

Cargo se conectará a internet, buscará el crate rand y también descargará cualquier otra cosa que ese crate necesite para funcionar. Este paso puede tardar un poco la primera vez, pero solo se hace una vez por versión.

Veremos algo como esto:

```plaintext
Updating crates.io index
Compiling rand v0.9.1
Finished dev [unoptimized + debuginfo] target(s) in 1.23s
```

¡Listo! Ahora tenemos acceso a la funcionalidad de `rand` en el proyecto.

### ¿Y si vuelves a ejecutar cargo build?

Si no hemos cambiado nada, veremos que el comando termina casi de inmediato. Esto es porque Cargo recuerda qué ya fue compilado, y no pierde tiempo recompilando cosas que siguen iguales.

Incluso si hacemos un cambio pequeño en el archivo `main.rs`, como cambiar un texto en pantalla, Cargo solo recompilará ese archivo. Tus dependencias (como `rand`) no se volverán a compilar, porque no han cambiado.

Esto hace que trabajar con proyectos en Rust sea muy eficiente.

### What is the `Cargo.lock` file?

Cuando ejecutamos `cargo build` por primera vez, Cargo crea un archivo llamado `Cargo.lock`. Su propósito es dejar registrado exactamente qué versiones de cada dependencia usaste, para que las próximas veces (incluso en otras computadoras o en el futuro), se use la misma configuración y tengas un build reproducible.

Este archivo es muy importante para garantizar que tu programa siempre se compile de la misma forma. Por eso, normalmente se sube junto con el código al repositorio (como en GitHub).

---

### Generating a Random Number

Ahora que ya agregamos `rand` como dependencia y todo está listo, podemos usarlo en nuestro código. Vamos a modificar el archivo `src/main.rs` para que genere un número aleatorio entre 1 y 100, que será el número que el jugador debe adivinar.

Pero antes, necesitamos entender un par de conceptos que usaremos en el código.

#### Importar lo que necesitamos

En Rust, si queremos usar funcionalidades de un `crate` externo (como `rand`), necesitamos importarlas al comienzo de tu archivo. Para generar un número aleatorio, usaremos dos cosas del crate `rand`:

El trait `Rng`, que nos da acceso a métodos para generar números aleatorios. Por el momento podemos pensar en `trait` como una especie de interfaz o codigo base que define un conjunto de métodos que un tipo debe implementar. En este caso, `Rng` es un trait que define métodos para generar números aleatorios.

La función `thread_rng()`, que nos da un generador de números aleatorios específico para el hilo actual del programa (de ahí el nombre `thread_rng`). Así se llamaba para versiones anteriores de `rand`, pero actualmente solo se llama `rng()`. Este generador es el que usaremos para crear nuestro número aleatorio.

Entonces, escribir esto al principio del `main.rs`:

```rust
use rand::Rng;
```

Este `use` le dice a Rust: “Voy a usar funcionalidades relacionadas con generación aleatoria que están dentro del crate `rand`”.

Ahora sí, generemos el número. Vamos a usar el método `random_range()` que está disponible gracias al trait `Rng`. Este método recibe un rango y nos devuelve un número aleatorio dentro de él. Por ejemplo:

```rust
let random_number = rand::rng().random_range(1..=100);
```

- `rand::rng()` nos da el generador de números aleatorios.
- `.random_range(1..=100)` le pide un número aleatorio desde 1 hasta 100, incluyendo el 100 (por eso usamos `..=` en lugar de `..`).

> Nota: `1..=100` es una sintaxis de rango inclusivo en Rust. Significa “desde el 1 hasta el 100, ambos incluidos”.

#### Mostrando el número secreto (por ahora)

Vamos a imprimir ese número secreto en la consola, solo para asegurarnos de que todo funciona. Luego lo ocultaremos, porque no tendría sentido mostrarlo en el juego final.

Agrega esto en la función `main()`:

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

---

## Comparing the Guess to the Secret Number

Ya tenemos la entrada del usuario y el número secreto generado aleatoriamente. Ahora toca compararlos.

📄 Archivo: `src/main.rs`

> ⚠️ Este código aún no compila, ya veremos por qué.

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --código omitido--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

Primero, usamos `use std::cmp::Ordering` para traer a nuestro alcance el tipo `Ordering` desde la biblioteca estándar. Este tipo es una enumeración (`enum`) que puede tener tres valores posibles:

- Less
- Greater
- Equal

Estos representan los tres posibles resultados al comparar dos valores.

Luego, usamos el método `.cmp()` para comparar `guess` (el intento del usuario) con `secret_number` (el número secreto). Este método devuelve uno de los tres valores de `Ordering`, y lo usamos con una expresión `match` para reaccionar según el resultado de la comparación.

### What is `match`?

Una expresión `match` en Rust permite tomar decisiones según el valor de una variable. Se compone de "brazos" (arms), y cada uno define una condición y el código a ejecutar si se cumple. Es similar a un `switch` en otros lenguajes, pero más poderoso y flexible.

Por ejemplo, si el usuario adivina 50 y el número secreto es 38, entonces `50.cmp(&38)` devuelve `Ordering::Greater`. El `match` verifica cada brazo:

- Compara con `Ordering::Less` → no coincide.
- Compara con `Ordering::Greater` → ¡coincide! → Imprime: "Too big!"

Y ahí se detiene, porque ya encontró una coincidencia.

### 💥 Pero... el código no compila

Si intentamos compilar este código con cargo build, veremos este error:

```bash
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                 --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
```

Significa que estamos intentando comparar dos cosas que no son del mismo tipo: `guess` es un `String` y `secret_number` es un número entero.

Rust tiene un sistema de tipos muy estricto. Aunque muchas veces puede adivinar (inferir) el tipo que queremos usar, aquí no puede comparar texto con números, y nos lo hace saber de inmediato.

La solución es transformar el texto que escribió el usuario en un número. Para eso, modificamos el código así: 

```rust
let mut guess = String::new();

io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

- `trim()` elimina espacios o saltos de línea. Por ejemplo, si el usuario escribe "`76\n`", lo deja como "`76`".
- `parse()` intenta convertir el texto en un número. Aquí especificamos que queremos un `u32` (entero sin signo de 32 bits).
- `expect()` se asegura de que, si parse falla (porque el usuario escribió letras, por ejemplo), se muestre un mensaje de error claro y el programa se detenga.

> 💡 Como usamos `let guess: u32`, estamos usando `shadowing` en Rust: reemplazamos la variable `guess` original (tipo `String`) por una nueva con el mismo nombre, pero ahora tipo `u32`. Esto es común en Rust y nos ayuda a mantener el código limpio y sin nombres confusos como `guess_str`. Lo que estamos haciendo es sobreescribir la variable `guess` con un nuevo valor y tipo.

Además, al hacer que `guess` sea un número, Rust deduce que `secret_number` también debe ser un `u32`, así que ya se pueden comparar directamente sin errores.

Ahora sí, ¡todo debería funcionar! Al ejecutar `cargo run`; incluso si el usuario pone espacios o salta de línea, el programa lo interpreta correctamente.

---

## Allowing Multiple Guesses with Looping

En Rust, la palabra clave `loop` nos permite repetir un bloque de código una y otra vez, infinitamente. Vamos a usarla para que el usuario tenga varias oportunidades de adivinar el número secreto:

```rust
// --snip--

println!("The secret number is: {secret_number}");

loop {
    println!("Please input your guess.");

    // --snip--

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

Como puedes ver, movimos todo el código desde que se le pide el número al usuario hasta la comparación dentro de un loop. Recuerda formatear el código con `cargo fmt` para que quede más legible o con alguna extensión de tu editor de texto favorito.

Ahora, si ejecutas el programa, este pedirá adivinar una y otra vez… ¡para siempre! Aunque eso introduce un pequeño problema: parece que no hay forma de salir del juego

### Quitting After a Correct Guess

El usuario puede salir del programa presionando `Ctrl + C`, pero eso no es muy elegante. Curiosamente, si el usuario escribe algo que no sea un número, el programa se va a detener con un error (porque intenta convertir texto en número y falla). Podemos aprovechar ese comportamiento como una "salida de emergencia".

Por ejemplo:

```bash
Please input your guess.
quit
thread 'main' panicked at 'Please type a number! ...
```

Si el usuario escribe quit o cualquier texto que no sea un número, el programa se cae. Pero eso no es lo ideal... tampoco queremos que se quede corriendo para siempre si el jugador ya acertó el número.

Podemos hacer que el programa termine automáticamente cuando el usuario adivine el número correctamente. Para eso usamos la instrucción `break`, que interrumpe el bucle:

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```

Este `break` hace que el juego termine de manera natural cuando el jugador gana. Como el bucle es la última parte de la función `main`, salir del bucle significa salir del programa.

### Handling Invalid Input

Ahora hagamos el juego más resistente: en vez de que se caiga cuando el usuario escribe algo que no es un número, lo ignoramos y simplemente volvemos a pedir otro intento.

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Please enter a valid number.");
        continue;
    }
};
```

Aquí usamos `match` para manejar el resultado del intento de convertir el texto (`guess.trim()`) en número. Si lo logra (`Ok(num)`), seguimos con ese número. Pero si falla (`Err(_)`), usamos continue para saltar a la siguiente vuelta del bucle, sin imprimir mensajes de error ni cerrar el programa. Simple y elegante.

Hora de probar el programa:

```bash
cargo run
Guess the number!
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
foo
Please enter a valid number.
Please input your guess.
61
You guessed: 61
You win!
```

Un último detalle, cuando estábamos probando el juego, mostrar el número secreto era útil, pero ahora arruina la sorpresa. Así que eliminamos esta línea:

```rust
// println!("The secret number is: {secret_number}");
```

## Full Code

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io; // std is the standard library, and io is the input/output module

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let random_number: u32 = rand::rng().random_range(1..=100);

    while true {
        println!("Guess a number between 1 and 100:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number.");
                break;
            }
        }
    }
}
```
