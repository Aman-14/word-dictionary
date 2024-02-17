pub mod backend;
pub mod file;

use backend::StorageBackend;
use std::collections::HashMap;

const HEADER_SIZE: usize = 70;

#[derive(Debug)]
pub struct Db<T: StorageBackend> {
    backend: T,
    indexes: HashMap<String, usize>,
    read_size: usize,

    pub version: usize,
    pub index_size: usize,
}

impl<T: StorageBackend> Db<T> {
    fn make_header(
        version: usize,
        index_size: usize,
        index_posi: usize,
        read_size: usize,
    ) -> Vec<u8> {
        let mut header_bytes = format!(
            "version={}\nindex_size={}\nindex_posi={}\nread_size={}",
            version, index_size, index_posi, read_size
        )
        .into_bytes();

        assert!(
            header_bytes.len() <= HEADER_SIZE,
            "Header bytes should be less than eq to HEADER_SIZE"
        );

        for _ in 0..(HEADER_SIZE - header_bytes.len()) {
            header_bytes.push(0)
        }

        return header_bytes;
    }

    pub fn new(b: T) -> Self {
        let mut buf = vec![0; HEADER_SIZE];

        b.read_at(&mut buf, 0).unwrap();
        let header = String::from_utf8(buf).unwrap();
        let parts = header.split('\n');

        let (mut version, mut index_size, mut index_posi, mut read_size) = (0, 0, 0, 0);

        for part in parts {
            let (name, value_string) = part.split_once("=").expect("Invalid header in the file");
            // println!("Name:`{}`\nValue:`{:?}`\n\n", name, value_string.as_bytes());

            let value: usize = value_string
                .trim_end_matches(char::from(0)) // trim null characters
                .parse()
                .unwrap_or(0);

            match name {
                "version" => version = value,
                "index_size" => index_size = value,
                "index_posi" => index_posi = value,
                "read_size" => read_size = value,
                _ => {
                    println!("Unhandled name: {}", name);
                }
            }
        }
        // println!("Vesion - {}", version);
        // println!("Index size - {}", index_size);
        // println!("Index_posi {}", index_posi);
        // println!("Read size - {}", read_size);
        buf = vec![0; index_size];
        b.read_at(&mut buf, index_posi as u64).unwrap();

        let indexes: HashMap<String, usize> =
            serde_json::from_str(&String::from_utf8(buf).unwrap()).unwrap();

        // println!("Indexes - {:?}", indexes);
        let s = Self {
            backend: b,
            indexes,
            version,
            index_size,
            read_size,
        };
        // println!("{:?}", s);
        return s;
    }

    // pub fn create() {
    //     let mut db = OpenOptions::new()
    //         .create(true)
    //         .write(true)
    //         .open(FILE_NAME)
    //         .unwrap();
    //
    //     let mut header_bytes = Self::make_header(1, 0, 0, 0);
    //     db.write(&mut header_bytes).unwrap();
    //     let mut indexes: HashMap<String, usize> = HashMap::new();
    //
    //     let data_file = File::open("filtered.csv").unwrap();
    //
    //     let data_reader = BufReader::new(data_file);
    //     let mut read_size = 0;
    //
    //     let mut cursor = HEADER_SIZE;
    //
    //     for line in data_reader.lines() {
    //         match line {
    //             Ok(line) => {
    //                 let parts = line.split_once(",");
    //                 if let Some((word, defination)) = parts {
    //                     let defination = defination
    //                         .trim_matches(|c| c == '"' || c == '\'')
    //                         .to_string()
    //                         + "\n";
    //                     println!("Word: {}, Defination: {}", word, defination);
    //                     let bytes = defination.as_bytes();
    //                     read_size = read_size.max(bytes.len());
    //                     indexes.insert(word.to_string().to_lowercase(), cursor);
    //                     cursor += db.write(bytes).unwrap();
    //                 }
    //             }
    //             Err(_) => {}
    //         }
    //     }
    //     println!("Cursor- {}", cursor);
    //     let index_posi = cursor;
    //     let encoded: Vec<u8> = serde_json::to_string(&indexes).unwrap().into_bytes();
    //     println!("Encoded - {:?}", encoded);
    //     let index_bytes_written = db.write(&encoded).unwrap();
    //     println!("Index bytes written {}", index_bytes_written);
    //
    //     header_bytes = Self::make_header(2, index_bytes_written, index_posi, read_size);
    //     db.seek(std::io::SeekFrom::Start(0)).unwrap();
    //     db.write(&header_bytes).unwrap();
    // }

    pub fn get_definition(&self, word: String) -> Option<String> {
        let address = self.indexes.get(&word)?;
        let mut buf = vec![0; self.read_size];

        self.backend.read_at(&mut buf, *address as u64).unwrap();
        let mut read_until = 0;
        for (i, ch) in buf.iter().enumerate() {
            if *ch == b'\n' {
                read_until = i;
                break;
            }
        }
        return Some(String::from_utf8(buf[..read_until].to_vec()).unwrap());
    }
}
