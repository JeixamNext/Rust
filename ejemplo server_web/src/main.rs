use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

// biblioteca estandar TCP y fs Módulo de sistema de archivos para leer el contenido de un archivo
//Usando TcpListener, podemos escuchar conexiones TCP
const ip_local:&str="127.0.0.1:7070";
//dirección local para flujos TCP entrantes.
fn main() {
    let listener = TcpListener::bind(ip_local).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
	//detener el programa si ocurren errores.
	// devuelve un iterador que nos da un secuencia de flujos tipo TcpStream
	handle_connection(stream);
    }
}

//  leer los datos del flujo TCP y imprímir respuesta y mostrar index html
fn handle_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("index.html").unwrap();
        let length = contents.len();
        let response = format!(

            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        stream.write_all(response.as_bytes()).unwrap();
    } else {

        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();
        let response = format!(

            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}

