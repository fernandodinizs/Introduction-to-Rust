
const SECONDS_IN_MINUTE: u32 = 60; // constante de segundos em um minuto
const MINUTES_IN_HOUR: u32 = 60; // constante de minutos em uma hora
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR; // constante de segundos em uma hora


fn main() { // incio escopo
   
    let total = 30; 
    let total_in_seconds = total * SECONDS_IN_HOUR; // calculando o total em segundos

    println!("total: {} segundos", total_in_seconds);
}

