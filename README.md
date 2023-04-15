# Calculadora con Matrices

![](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

> Autores: Tomás Badenes y Santiago Fernandez

![Video de demostracion](doc-assets/demo.gif)

## Descripción

Este programa fue el resultado de un desafío de un docente de cátedra de matemática. La consigna era:

> 1. Escribir un código que dadas dos matrices analice si pueden sumarse y en caso afirmativo calcule la suma
> 2. Escribir un código que dadas dos matrices analice si el producto está definido y en caso afirmativo lo calcule
> 3. Escribir un código que dada una matriz analice si tiene inversa y en caso afirmativo la calcule
> 4. Implemente un programa que dado un sistema de ecuaciones lineales analice la compatibilidad del sistema

Nosotros hemos decidido llevarlo un paso mas allá y crear una calculadora completa, escrita en Rust, que integre las soluciones a los cuatro problemas propuestos y un poco mas. Sus características se detallan [abajo](#características), así como las [instrucciones de instalación y uso](#utilización).

## Características

La calculadora funciona mediante comandos. La sintaxis general es: `<COMANDO> [parámetros]`

Para obtener una lista de los posibles comandos, utilice el comando `ayuda`

### Operaciones entre escalares y matrices

Esta calculadora es capaz de resolver calculos combinados utilizando operaciones entre matrices y escalares. Las operaciones soportadas son:

- `*`: Multiplicación
- `/`: División
- `+`: Suma
- `-`: Resta (o suma con la `-1 * el sustraendo`)
- `^`: Potencia
- `T`: Transpuesta de una matriz (sintaxis: `<matriz> T`)
- `DET`: Determinante de una matriz (sintaxis: `<matriz> DET`)
- `INV`: Inversa de una matriz (sintaxis: `<matriz> INV`)

### Resolucion de cálculos combinados

Utilizando el comando `ecu` y como parametro un cadena de operandos y operadores separados por espacios, la calculadora resolverá el cálculo si tiene solución, o mostrará un mensaje de error en caso de expresiones malformadas o operaciones no definidas.

Por ejemplo: `((((Matriz A + B) * (Matriz A al cuadrado)) <- Transponer) <- Determinante) + PI`

```
>>> ecu ( ( A + B ) * ( A ^ 2 ) T ) DET + PI
Resultado: -186.8585
```

### Declaracion y almacenamiento de variables

Utilizando el comando `mostrar` se pueden ver las variables almacenadas. La calculadora incluye algunas por defecto:

```
>>> mostrar
PI = 3.1415

B =
3 4.5
8 2

A =
1 2
3 4

C = 0
```

También se le puede exigir al comando mostrar que filtre por nombre de variable:

```
>>> mostrar A PI E
A =
1 2
3 4

PI = 3.1415

La variable `E` no está definida
```

Estos identificadores pueden luego ser utilizados en las expresiones `ecu`

### Sistemas de ecuaciones

Mediante el comando `ecsis` se puede ingresar un sistema de ecuaciones en la forma de la matriz expandida en el vector resultado (A|b) para determinar si este tiene solución o no.

## Utilización

### Instalación desde código fuente

1. Clonar el repositorio:

```
git clone https://github.com/b-Tomas/calculadora.git
```

O utilizar el botón verde para descargar un comprimido `.zip`

2. Descargar e instalar la versión de rust para tu sistema operativo desde la [página oficial](https://www.rust-lang.org/tools/install)

3. Compile el proyecto

```
cargo build
```

4. Compruebe que los tests de unidad:

```
$ cargo test
running 21 tests
test exp_interpreter::tests::test_postfix_to_tree ... ok
test exp_interpreter::tests::test_infix_to_postfix ... ok
test math::tests::compatible_determinado ... ok
test math::tests::identity_matrix ... ok
test math::tests::determinant ... ok
test math::tests::incompatible_equation ... ok
test math::tests::mat_pow ... ok
test math::tests::matrix_multiplication ... ok
test math::tests::inverse_test ... ok
test math::tests::matrix_sub ... ok
test math::tests::matrix_sum ... ok
test math::tests::multiplication_by_scalar ... ok
test math::tests::transposed ... ok
test exp_interpreter::tests::test_solve ... ok
test math::tests::test_adj ... ok
test math::tests::undetermined ... ok
test math::tests::very_incompatible ... ok
test structs::tests::create_empty_matrix ... ok
test structs::tests::create_matrix_from_data ... ok
test structs::tests::equals ... ok
test structs::tests::is_squared ... ok

test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

5. Inicie el programa:

```
cargo run
```

### Instalación de binarios

Visite la página de [releases](https://github.com/b-Tomas/calculadora/releases) para descargar el programa compilado para su plataforma, aunque se recomienda instalar de código fuente para tener disponible la última versión del programa.

## Acerca de las soluciones

La lógica responsable de la solución de los problemas matemáticos se encuentra en el archivo `src/math.rs`. Dado que se trataba de trabajo práctico de matemáticas, decidimos mantenernos alejados de librerías externas que nos faciliten los cálculos y programamos las soluciones desde cero.

Además, fue nuestro primer contacto con el lenguaje Rust. Si bien el código no es perfecto o el más prolijo, creemos que es bastante aceptable habiendo tenido una semana.
