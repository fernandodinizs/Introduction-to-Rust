fn main() { // incio escopo
    let mut total = 30; //mut deixa a variavel mutavel  
    let segundos = 60.20;

    println!("total: {} horas, e {} segundos", total, segundos);

    {
        let total = total + 20 ; // com as chaves tudo feito dentro delas não muda fora
        println!("total: {} horas", total);
    }

    total = 40;
    println!("total: {} horas", total);
    
    let total = "quarente"; //para redefinir a tipagem da variavel é preciso redefinir a variavel com let   
    println!("total: {} horas", total);
}

// i32 = inteiro de 32 bits
// f64 = ponto flutuante de 64 bits
//let total: i32 = 30;
// let segundos: f64 = 60.20;


// é possivel definir o sugb escopo com:

// {
//     let total = 50;
// }

// ou:

// fn inner() {
//     let total = 50;
// }