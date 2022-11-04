fn main() {
    // Creamos un nuevo canal usando el mpsc::channel 
    // mpsc Productor múltiple, consumidor único
    // mpsc::channel devuelve una tupla
    let (tx, rx) = mpsc::channel();
    // thread::spawnpara crear un nuevo hilo
    let tx1 = tx.clone(); //crear varios subprocesos que envíen valores al mismo receptor
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
	//una pausa entre cada una de las llamandas enviadas individualmente al hilo principal 
        }
    });



    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

