fn factorial_de(valor:f32) -> f32 {
    
    let mut resultado : f32 = 1.0;
    let mut contador : i8 = 1;

    loop{

        if contador as f32 <= valor {

            resultado *= contador as f32;
            contador+=1;
        }

        else{

            break;
        }
    }
    
    return resultado;

}

fn seno(x:f32,limite_final:u8) -> f32 {

    let mut n : u8 = 0;
    let mut sumando : f32 = 0.0;
  

    loop{

        if n <= limite_final {

            let signo = (-1_f32).powf(n as f32);
            let potencia = x.powf(2.0 * n as f32 + 1.0);
            let factorial = factorial_de(2.0 * n as f32 + 1.0); 
            
            sumando += signo * (potencia / factorial); 

            n+=1;
        }
        else {
            break;
        }
    }

    return sumando;
}

fn coseno(x : f32, limite_final : u8) -> f32 {

    let mut sumando : f32 = 0.0;
    let mut n : u8 = 0;

    loop{

        if n <= limite_final {

            let signo = (-1_f32).powf(n as f32);
            let potencia = x.powf(2.0 * n as f32);
            let factorial = factorial_de(2.0 * n as f32);
            sumando += signo * (potencia/factorial);
            n+=1;
        }

        else {
            break;
        }
    }

    return sumando;
}


fn tangente(x : f32, limite : u8) -> f32 {

    return seno(x,limite)/coseno(x,limite);
}



fn main(){

    println!("{:.4}",seno(0.7853,10));
    println!("{:.4}",coseno(0.7853,10));
    println!("{:.2}",tangente(0.7853,10));


}
