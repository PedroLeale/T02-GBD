mod dynamic_hash;
mod register;
//Altere estas constantes se quiser rodar com outros valores
const BUFFER_SIZE: u32 = 1000; // Contado em registros ( cada um de 100 bytes )
const NUMBER_OF_REGISTERS: u32 = 1000;
const INITIAL_CAPACITY: usize = 1000;
const FILE_NAME: &str = "arquivo_sem_index_por_hash_dinamico";
const HASH_TABLE_FILE_NAME: &str = "tabela_hash_dinamica";

fn main() {
    let arquivo = register::Arquivo::new(NUMBER_OF_REGISTERS, BUFFER_SIZE, FILE_NAME.to_owned());
    arquivo.write_in_file();
    println!("Register number 50: {:?}", arquivo.sequential_read(10));
    println!("Register number 40: {:?}", arquivo.sequential_read(11));

    let mut hash_table = match dynamic_hash::DynamicHashTable::new(INITIAL_CAPACITY, HASH_TABLE_FILE_NAME.to_owned()) {
        Ok(hash_table) => hash_table,
        Err(_) => panic!("Erro ao criar tabela hash"),
    };
    println!(
        "{:?}",
        hash_table.insert(10, arquivo.sequential_read(10).unwrap().get_nome())
    );
    println!(
        "{:?}",
        hash_table.insert(11, arquivo.sequential_read(11).unwrap().get_nome())
    );

    println!("-------------------------");
    println!("Hash table nseq 10: {:?}", hash_table.read_key(10));
    println!("Hash table nseq 11: {:?}", hash_table.read_key(11));
    println!("Hash table remove 10: {:?}", hash_table.remove_key(10));
    println!("Hash table nseq 10: {:?}", hash_table.read_key(10));

    println!("-------------------------");
}
