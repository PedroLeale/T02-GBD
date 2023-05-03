mod dynamic_hash;
mod register;

fn main() {
    let arquivo = register::Arquivo::new(100, 100, "test version".to_owned());
    arquivo.write_in_file();
    println!("arquivo length: {}", arquivo.get_file_size());
    println!("Register number 50: {:?}", arquivo.sequential_read(50));
    let mut hash_table: dynamic_hash::DynamicHashTable<u32, [char; 96]> =
        dynamic_hash::DynamicHashTable::new();
    for i in 0..100 {
        let registro = arquivo.sequential_read(i);
        if let Some(registro) = registro {
            if hash_table.insert(registro.get_nseq(), registro.get_nome()).is_err() {
                println!("Erro ao inserir registro {}", i);
            }
        } else {
            println!("Registro {} não encontrado", i);
        }
    }
    println!("Hash table nseq 50: {:?}", hash_table.get(&50));
    if hash_table.remove(&50).is_err() {
        println!("Erro ao remover registro 50");
    }
    println!("Hash table nseq 50: {:?}", hash_table.get(&50));
}
