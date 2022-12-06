
#[derive(Debug)]
struct Persona {
    identificador: i32
}

impl Clone for Persona {
    fn clone(&self) -> Persona {
        Self {
            identificador: self.identificador.clone() 
        }
    }
}

fn main(){

    let persona = Persona { identificador: 466 };
    let otra_persona = persona.clone();

    print!("{:?} \n", otra_persona);
    print!("{:?} \n", persona);
}