mod bplustree;
mod register;

fn main() {
    let mut arquivo = register::Arquivo::new(100, 100, "100kb version".to_owned());
    arquivo.write_in_file();
    println!("arquivo length: {}", arquivo.get_file_size());
    println!("Register number 50: {:?}", arquivo.sequential_read(50));
    match arquivo.delete_register(50){
        true => println!("registro deletado"),
        false => println!("registro não deletado"),
    }
    println!("Register number 50: {:?}", arquivo.sequential_read(50));
    println!("arquivo length: {}", arquivo.get_file_size());
}
