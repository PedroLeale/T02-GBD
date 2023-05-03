use rand::{distributions::Alphanumeric, Rng}; //rand = "0.8.5"
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

#[derive(Debug)]
pub struct Registro {
    nseq: u32,
    nome: [char; 96],
}

#[allow(dead_code)]
impl Registro {
    fn new(nseq: u32) -> Registro {
        let rng = rand::thread_rng();
        let arr: [char; 96] = rng
            .sample_iter(Alphanumeric)
            .take(96)
            .map(char::from)
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        Registro { nseq, nome: arr }
    }

    fn update(&mut self, nseq: u32) {
        self.nseq = nseq;
        let rng = rand::thread_rng();
        self.nome = rng
            .sample_iter(Alphanumeric)
            .take(96)
            .map(char::from)
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
    }

    pub fn get_nseq(&self) -> u32 {
        self.nseq
    }

    pub fn get_nome(&self) -> [char; 96] {
        self.nome
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Arquivo {
    file_name: String,
    file_size: u32,
    buffer_size: u32,
}

#[allow(dead_code)]
impl Arquivo {
    //Buffer size is measured in amount of registers, every register has 100 bytes
    pub fn new(number_of_registers: u32, buffer_size: u32, file_name: String) -> Arquivo {
        Arquivo {
            file_name,
            file_size: number_of_registers,
            buffer_size,
        }
    }

    pub fn write_in_file(&self) {
        let mut end = self.file_size / self.buffer_size;
        if self.file_size % self.buffer_size != 0 {
            end += 1;
        }
        let mut break_argument = self.file_size as i64;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.file_name)
            .unwrap();

        for i in 0..end {
            let mut registros: Vec<Registro> = Vec::new();
            for j in 0..self.buffer_size {
                let registro = Registro::new(i * self.buffer_size + j);
                registros.push(registro);

                if break_argument <= 0 {
                    //Only used in cases the file size is not a multiple of the buffer size
                    break;
                }
            }
            break_argument -= self.buffer_size as i64;
            let mut vec = Vec::new();
            for registro in &registros {
                let mut nseq = registro.nseq.to_be_bytes().to_vec();
                let mut nome = registro.nome.iter().collect::<String>().into_bytes();
                vec.append(&mut nseq);
                vec.append(&mut nome);
            }
            file.write_all(&vec).unwrap();
        }
    }

    //Will make a sequential read until it finds the register with the nseq given
    pub fn sequential_read(&self, nseq: u32) -> Option<Registro> {
        let mut file = self.get_file();
        let mut buffer = [0u8; 100];
        if nseq > self.file_size {
            return None;
        }
        loop {
            match file.read(&mut buffer) {
                Ok(0) => return None,
                Ok(_) => {
                    let (nseq_buf, nome_buf) = buffer.split_at(std::mem::size_of::<u32>());
                    let registro_nseq = u32::from_be_bytes(nseq_buf.try_into().unwrap());
                    if registro_nseq == nseq {
                        let nome = String::from_utf8_lossy(nome_buf).to_string();
                        let registro = Registro {
                            nseq,
                            nome: nome
                                .chars()
                                .take(96)
                                .collect::<Vec<char>>()
                                .try_into()
                                .unwrap(),
                        };
                        return Some(registro); //Returns desired register
                    }
                }
                Err(_) => return None, // read error
            }
        }
    }

    pub fn insert_at_end(&mut self) {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file_name)
            .unwrap();
        let registro = Registro::new(self.file_size);
        let mut nseq = registro.nseq.to_be_bytes().to_vec();
        let mut nome = registro.nome.iter().collect::<String>().into_bytes();
        let mut vec = Vec::new();
        vec.append(&mut nseq);
        vec.append(&mut nome);
        self.file_size += 1;
        file.write_all(&vec).unwrap();
    }

    pub fn update_random(&self, nseq: u32) {
        let register = Registro::new(nseq);
        let mut file = self.get_file();
        let pos = (nseq as usize) * 100;
        file.seek(SeekFrom::Start(pos as u64)).unwrap();
        let mut nseq = register.nseq.to_be_bytes().to_vec();
        let mut nome = register.nome.iter().collect::<String>().into_bytes();
        let mut vec = Vec::new();
        vec.append(&mut nseq);
        vec.append(&mut nome);
        file.write_all(&vec).unwrap();
    }

    pub fn delete_register(&mut self, nseq: u32) -> bool {
        let mut file = self.get_file();
        let mut buffer = [0u8; 100];
        let mut read_pos = 0;
        let mut write_pos = 0;
        let mut found = false;

        loop {
            let read_result = file.read(&mut buffer);
            match read_result {
                Ok(0) => break,
                Ok(_) => {
                    let (nseq_buf, _) = buffer.split_at(std::mem::size_of::<u32>());
                    let registro_nseq = u32::from_be_bytes(nseq_buf.try_into().unwrap());
                    if registro_nseq != nseq {
                        if read_pos != write_pos {
                            file.seek(SeekFrom::Start(write_pos)).unwrap();
                            file.write_all(&buffer).unwrap();
                        }
                        write_pos += 100;
                    } else {
                        found = true;
                    }
                    read_pos += 100;
                }
                Err(_) => return false, // read error
            }
        }
        match file.flush() {
            Ok(_) => found,
            Err(_) => false,
        }
    }

    pub fn get_file_size(&self) -> u64 {
        let path = Path::new(&self.file_name);
        let metadata = std::fs::metadata(path).unwrap();
        metadata.len()
    }

    pub fn get_file(&self) -> File {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file_name)
            .unwrap();
        file
    }
}
