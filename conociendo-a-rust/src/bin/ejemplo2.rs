
struct Libros {
    titulo:String,
    autor: String,
    identificador: i32,
}

fn mostrarLibro(libro: &Libros) {
    println!("Libro título: {}", libro.titulo);
    println!("Libro autor: {}", libro.autor);
    println!("Libro id: {}", libro.identificador);
}

fn modificarTitulo(libro: &mut Libros, titulo: String){
    libro.titulo = titulo;
}

fn main() {

    let mut primerLibro: Libros = Libros { titulo: String::from("Don Quijote de la Mancha"), 
                                autor: String::from("Miguel de Cervantes"), 
                                identificador: 1};
    
    modificarTitulo(&mut primerLibro, String::from("Don Quijote de la Mancha - Edición conmemorativa"));
    
    mostrarLibro(&primerLibro);
    
}
