use serde::{Serialize, Deserialize};
use std::io;
//use serde_json::Value; ya no lo necesitamos por que cambiamos de structura por una propia
fn main() {
    let mut coin = String::new();
    //TODO: creamos un mensaje para que el Usuario pueda ver que dato ingresar
    println!("¿Que Crypto moneda deseas consultar?");
    //TODO LIBRERIA DE ENTRADA STD::IO para evitar controlar este error de ingreso de datos vamso a usar .expect()
    std::io::stdin().read_line(&mut coin).expect("Ocurrio un Error en el std::io");
    //let _ = std::io::stdin().read_line(&mut coin).expect("Ocurrio un Error en el std::io"); puedo colocar el _ para decirle a rust que no me interesa el resultado  o variable
    //let response: Result<usize, std::io::Error> = std::io::stdin().read_line(&mut coin).expect("Ocurrio un Error en el std::io");
    //Realizamos la peticion mediante la funcion get_fetch()
    let response = get_fetch(&coin);//referecia al valor 
    match response {
        Ok(value) => println!("el valor de la respuesta es : ${}", value),//podriamos pasar el valor a pesos
        Err(error) => println!("Ocurrio un error en la peticion: {}", error),
    }
    let mut buffer = String::new();
    println!("¡Precione enter para terminar!");
    let _ = io::stdin().read_line(&mut buffer);//Espera que el usuario presione enter para salir
    //TODO MANEJO DE ERRORES
   /*  match response {
        Ok(_bytes) => {
            //println!("el numero de bytes leidos {bytes}");//nos devuelve el nuemro de bytes leidos de la respuesta
            println!("Moneda seleccionada: {}", coin);
            let res = get_fetch(&coin);// como la funcion no necesita modificar el valor solo usarlo para la consulta se pasa una referencia
            println!("el precio es: {}", res);
        },
        Err(error) => println!("El error: {}", error)
    } */
}

//TODO funcion con peticion con libreria http y serializacion para el json
fn get_fetch(coin: &str)-> Result<String, ureq::Error> {
  //Necesitamos realizar un Request para descargar paquetes o librerias usamos creates.io
  let body: String = ureq::get(&format!("https://api.coingecko.com/api/v3/coins/{}?localization=false", coin))
  .set("Example-Header", "header value")
  .call()?
  .into_string()?;
//let parse : Coin = serde_json::from_str(&body).unwrap();
let coin_data : Coin = serde_json::from_str(&body).unwrap();
Ok(coin_data.market_data.current_price.ars.to_string())
  //String::from("prueba")//from me devuelve un texto que yo formule
}

//TODO estructura para evitar cometer errores en la serializacion
//TODO: para realizar esto necesitamos serde y tomar solo una parte osea un features ahora indicamos a nuestra structura que puede ser serialziada o deserealizada
#[derive(Serialize, Deserialize, Debug)]
struct Coin {
    id: String,
    symbol: String,
    name: String,
    market_data: MarketData//paso mi otra structura como typo 
}

//TODO otra structura para otro campo del json que pedimos
#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    current_price: Prices,//igual que arriba creo otra structura para los precios se podria hacer un map
}
//TODO structura para agregar desde la api los campos ed precios
#[derive(Serialize, Deserialize, Debug)]
struct Prices {
    usd: f32,
    ars: f32,
}

//para construir la aplicacion una vez terminamos usamos el cargo build --release