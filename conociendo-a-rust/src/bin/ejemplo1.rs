

fn main(){
    
    let numeros_primeros: [i32; 5] = [2, 3, 5, 7, 13];
    let size: usize = numeros_primeros.len();
    for i in 0..size {
        println!("{}", numeros_primeros[i]);
    }

}