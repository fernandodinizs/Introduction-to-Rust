fn main() { // incio escopo
   let mut number = [1, 2, 3];

   println!("{:?}", number);
}

// tipo padrão em numericos(inteiro) é i32
// tipo padrão em numericos(float) é f64

//Formas de ler numericos em Rust
//  let x = 5; 
//  let x = 432_452_645_43;  Rust ignora os _ e le como se fosse somente um numero grande
//  let x = 0xff; Reconhece como hexadecimal 
//  let x = 0o77; Reconhece como octal 
//  let x = 0b1111; Reconhece como binário
//  let x = b'A'; Reconhece como byte 

//compost type

//Tupla:
// Não é possivel alterar o tipo de valor de uma tupla e nem o tamanho dela, mas é possivel alterar o valor de cada elemento da tupla
// let number (i32, i32, f64) = (1, 2, 3.5);
// println!("{:?}", number.0); // Para acessar o valor do primeiro elemento da tupla
// o tipo dos valores da tupla e dada de modo individual
// let (a, b, c) = number; //desestruturação da tupla

//let mut number = (1, 2, 3);
//   number.0 = 5; //alterando o valor do primeiro elemento da tupla
//   number = (5, 2, 3); //alterando o valor de todos elementos da tupla

//Array:
// Não permite diferentes tipos de valores, mas é mutavel que nem as tuplas
// let number [i32; 3] = [1, 2, 3]; 
// No array é passado o tipo dos valores e a quantidade de elementos do array
// println!("{:?}", number[0]); // Para acessar o valor do primeiro elemento do array


//let mut number = [1, 2, 3];
//   number[0] = 5; //alterando o valor do primeiro elemento do array

//let number = [1, 2, 3];
//   println!("{:?}", &number[0..2]); //para acessar uma parte do array