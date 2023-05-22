mod dynamic_hash;
mod register;
//Altere estas constantes se quiser rodar com outros valores
const BUFFER_SIZE: u32 = 10; // Contado em registros ( cada um de 100 bytes )
const NUMBER_OF_REGISTERS: u32 = 100;
const INITIAL_CAPACITY: usize = 3;
const FILE_NAME: &str = "arquivo_sem_index_por_hash_dinamico";
const HASH_TABLE_FILE_NAME: &str = "hash_dinamico_alternativa_1";

fn main() {
    let arquivo = register::Arquivo::new(NUMBER_OF_REGISTERS, BUFFER_SIZE, FILE_NAME.to_owned());
    arquivo.write_in_file();
    println!("Register number 50: {:?}", arquivo.sequential_read(10));
    println!("Register number 40: {:?}", arquivo.sequential_read(11));

    let mut hash_table = match dynamic_hash::DynamicHashTable::new(INITIAL_CAPACITY, HASH_TABLE_FILE_NAME.to_owned()) {
        Ok(hash_table) => hash_table,
        Err(_) => panic!("Erro ao criar hash"),
    };
    println!(
        "{:?}",
        hash_table.insert(10, arquivo.sequential_read(10).unwrap().get_nome())
    );
    println!(
        "{:?}",
        hash_table.insert(11, arquivo.sequential_read(11).unwrap().get_nome())
    );
    println!(
        "{:?}",
        hash_table.insert(12, arquivo.sequential_read(12).unwrap().get_nome())
    );
    println!(
        "{:?}",
        hash_table.insert(13, arquivo.sequential_read(13).unwrap().get_nome())
    );

    println!("-------------------------");
    println!("Hash table nseq 10: {:?}", hash_table.read_key_value(10));
    println!("Hash table nseq 11: {:?}", hash_table.read_key_value(11));
    println!("Hash table remove 10: {:?}", hash_table.remove_key_value(10));
    println!("Hash table nseq 10: {:?}", hash_table.read_key_value(10));
    println!("-------------------------");

    hash_table.print_all_table();

}
