extern crate gtk;

use gtk::prelude::*; // importa todo lo basico
use gtk::{Window,WindowType,Label};

pub fn main () { 
     if gtk::init().is_err(){ //inicializa GTK antes de hacer nada
	 panic!("no puede iniciar GTK");
	}
     let window=Window::new(WindowType::Toplevel);
     // destruye la ventana al salir
     window.connect_delete_event(|_,_| {gtk::main_quit();Inhibit(false)});
     window.set_title("ejemplo ventana");
     window.set_default_size(300,70);
     let label=Label::new(Some("aqui texto"));
     window.add(&lavel);
     window.show_all();
     gtk::main();
}
