use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

// biblioteca estandar TCP y fs Módulo de sistema de archivos para leer el contenido de un archivo
//thread para creacion de hilos
//Usando TcpListener, podemos escuchar conexiones TCP
use server_web::ThreadPool;
// uso del submodulo biblioteca
// cargo doc --open genera la documentacion
const IP_LOCAL:&str="127.0.0.1:7070";
//dirección local para flujos TCP entrantes.

        //detener el programa si ocurren errores.
        // devuelve un iterador que nos da un secuencia de flujos tipo TcpStream
        // thread::spawn creará un nuevo hilo pero seran ilimitados

fn main() {

    let listener = TcpListener::bind(IP_LOCAL).unwrap();
    let pool = ThreadPool::new(4);
    //numero limitado de hilos
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

}



//  leer los datos del flujo TCP y imprímir respuesta y mostrar index html

fn handle_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response =
      format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

