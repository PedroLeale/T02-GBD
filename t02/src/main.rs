mod dynamic_hash;
mod register;

//Altere estas constantes se quiser rodar com outros valores
const BUFFER_SIZE: u32 = 100;
const NUMBER_OF_REGISTERS: u32 = 100;
const FILE_NAME: &str = "arquivo_sem_index_por_hash_dinamico";
const HASH_TABLE_FILE_NAME: &str = "tabela_hash_dinamica";
/*
Dependencias no arquivo Cargo.toml
rand = "0.8.5"
bincode = "1.3.3"
serde = {version = "1.0.160", features = ["derive"]}
serde_arrays = "0.1.0"
*/

fn main() {
    let arquivo = register::Arquivo::new(NUMBER_OF_REGISTERS, BUFFER_SIZE, FILE_NAME.to_owned());
    arquivo.write_in_file();
    println!("Register number 50: {:?}", arquivo.sequential_read(50));
    println!("Register number 40: {:?}", arquivo.sequential_read(40));
    let mut hash_table: dynamic_hash::DynamicHashTable = dynamic_hash::DynamicHashTable::new();

    for i in 0..NUMBER_OF_REGISTERS {
        let registro = arquivo.sequential_read(i);
        if let Some(registro) = registro {
            if hash_table
                .insert(registro.get_nseq(), registro.get_nome())
                .is_err()
            {
                println!("Erro ao inserir registro {}", i);
            }
        } else {
            println!("Registro {} não encontrado", i);
        }
    }
    println!("Hash table nseq 40: {:?}", hash_table.get(&40));

    if hash_table.remove(&40).is_err() {
        println!("Erro ao remover registro 40");
    }

    println!("Hash table nseq 40, para mostrar que removeu: {:?}", hash_table.get(&40));

    if hash_table.save_to_file(HASH_TABLE_FILE_NAME).is_err() {
        println!("Erro ao salvar hash table");
    }

    if dynamic_hash::read_from_file(HASH_TABLE_FILE_NAME).is_none() {
        println!("Erro ao ler hash table do arquivo");
    } else {
        let hash_table_from_file = dynamic_hash::read_from_file(HASH_TABLE_FILE_NAME).unwrap();
        println!("Hash table nseq 40 depois de ler do arquivo salvo: {:?}", hash_table_from_file.get(&40));
        println!("Hash table nseq 50 depois de ler do arquivo salvo: {:?}", hash_table_from_file.get(&50));
    }

}
