extern crate gtk;

use gtk::prelude::*; // importa todo lo basico
use gtk::{Window,WindowType,Label,Entry,Box as GtkBox,Orientation};

pub fn init_v () { 
     if gtk::init().is_err(){ //inicializa GTK antes de hacer nada
	 panic!("no puede iniciar GTK");
	}
     let window=Window::new(WindowType::Toplevel);
     // destruye la ventana al salir
     window.connect_delete_event(|_,_| {gtk::main_quit();Inhibit(false)});
     window.set_title("ejemplo ventana");
     window.set_default_size(300,70);
     let label=Label::new(Some("aqui texto"));

     // crear VBOX
     let bx=GtkBox::new(Orientation::Vertical,10);
     let entry=Entry::new();    
     // activa la señal de funciones anonimas
     entry.connect_activate(|x|println!("{}",x.get_text()));
     //añadir el label y la entrada
     bx.pack_start(&label,false,false,0); // no expasible y 0 padding
     bx.pack_start(&entry,false,false,0);
     window.add(&bx);
     window.show_all();
     gtk::main();
}