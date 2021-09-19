use super::chunk_type::ChunkType;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    ctype: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(ctype: ChunkType, data: Vec<u8>) -> Self {
        let length = data.len() as u32;
        let mut chunk = Chunk {
            length,
            ctype,
            data,
            crc: 0,
        };

        chunk.crc = chunk.crc();
        chunk
    }

    fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.ctype
    }

    fn data(&self) -> &[u8] {
        &self.data[..]
    }
    
    fn crc(&self) -> u32 {
        use crc::crc32;
        crc32::checksum_ieee(&self.as_bytes()[4..&self.as_bytes().len() - 4])
    }

    pub fn data_as_string(&self) -> anyhow::Result<String> {
        if self.length == 0 { return Err(anyhow::anyhow!("No data to be printed")); }
        Ok(String::from_utf8(self.data.clone())?)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length.to_be_bytes().iter()
            .chain(self.ctype.bytes().iter())
            .chain(self.data().iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect::<Vec<u8>>()
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Length: {}", self.length)?;
        writeln!(f, "Chunk Type: {}", self.ctype)?;
        writeln!(f, "Data: {}", self.data_as_string().unwrap_or(String::from("Invalid UTF-8")))?;
        writeln!(f, "Crc: {}", self.crc)?;
        Ok(())
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;
    fn try_from(slice: &[u8]) -> anyhow::Result<Chunk> {
        let len_bytes: [u8; 4] = slice[..4].try_into()?;
        let length = u32::from_be_bytes(len_bytes);

        let ctype_bytes: [u8; 4] = slice[4..8].try_into()?;
        let ctype = ChunkType::try_from(ctype_bytes)?;

        let data = slice[8..slice.len() - 4].to_vec();

        let crc_bytes: [u8; 4] = slice[slice.len() - 4..].try_into()?;
        let crc = u32::from_be_bytes(crc_bytes);

        let maybe_chunk = Chunk {
            length,
            ctype,
            data,
            crc,
        };

        if maybe_chunk.crc() != crc {
            return Err(anyhow::anyhow!("Checksum does not match"));
        }

        Ok(maybe_chunk)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::chunk_type::ChunkType;
//     use std::str::FromStr;

//     fn testing_chunk() -> Chunk {
//         let data_length: u32 = 42;
//         let chunk_type = "RuSt".as_bytes();
//         let message_bytes = "This is where your secret message will be!".as_bytes();
//         let crc: u32 = 2882656334;

//         let chunk_data: Vec<u8> = data_length
//             .to_be_bytes()
//             .iter()
//             .chain(chunk_type.iter())
//             .chain(message_bytes.iter())
//             .chain(crc.to_be_bytes().iter())
//             .copied()
//             .collect();

//         Chunk::try_from(chunk_data.as_ref()).unwrap()
//     }

//     #[test]
//     fn test_new_chunk() {
//         let c1 = Chunk::new(ChunkType::from_str("RuSt").unwrap(), "This is where your secret message will be!".into());
//         let c2 = testing_chunk();
//         assert_eq!(c1.length, c2.length);
//         assert_eq!(c1.ctype, c2.ctype);
//         assert_eq!(c1.data, c2.data);
//         assert_eq!(c1.crc, c2.crc);
//     }

//     #[test]
//     fn test_chunk_length() {
//         let chunk = testing_chunk();
//         assert_eq!(chunk.length(), 42);
//     }

//     #[test]
//     fn test_chunk_type() {
//         let chunk = testing_chunk();
//         assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
//     }

//     #[test]
//     fn test_chunk_string() {
//         let chunk = testing_chunk();
//         let chunk_string = chunk.data_as_string().unwrap();
//         let expected_chunk_string = String::from("This is where your secret message will be!");
//         assert_eq!(chunk_string, expected_chunk_string);
//     }

//     #[test]
//     fn test_chunk_crc() {
//         let chunk = testing_chunk();
//         assert_eq!(chunk.crc(), 2882656334);
//     }

//     #[test]
//     fn test_valid_chunk_from_bytes() {
//         let data_length: u32 = 42;
//         let chunk_type = "RuSt".as_bytes();
//         let message_bytes = "This is where your secret message will be!".as_bytes();
//         let crc: u32 = 2882656334;

//         let chunk_data: Vec<u8> = data_length
//             .to_be_bytes()
//             .iter()
//             .chain(chunk_type.iter())
//             .chain(message_bytes.iter())
//             .chain(crc.to_be_bytes().iter())
//             .copied()
//             .collect();

//         let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

//         let chunk_string = chunk.data_as_string().unwrap();
//         let expected_chunk_string = String::from("This is where your secret message will be!");

//         assert_eq!(chunk.length(), 42);
//         assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
//         assert_eq!(chunk_string, expected_chunk_string);
//         assert_eq!(chunk.crc(), 2882656334);
//     }

//     #[test]
//     fn test_invalid_chunk_from_bytes() {
//         let data_length: u32 = 42;
//         let chunk_type = "RuSt".as_bytes();
//         let message_bytes = "This is where your secret message will be!".as_bytes();
//         let crc: u32 = 2882656333;

//         let chunk_data: Vec<u8> = data_length
//             .to_be_bytes()
//             .iter()
//             .chain(chunk_type.iter())
//             .chain(message_bytes.iter())
//             .chain(crc.to_be_bytes().iter())
//             .copied()
//             .collect();

//         let chunk = Chunk::try_from(chunk_data.as_ref());

//         assert!(chunk.is_err());
//     }

//     #[test]
//     pub fn test_chunk_trait_impls() {
//         let data_length: u32 = 42;
//         let chunk_type = "RuSt".as_bytes();
//         let message_bytes = "This is where your secret message will be!".as_bytes();
//         let crc: u32 = 2882656334;

//         let chunk_data: Vec<u8> = data_length
//             .to_be_bytes()
//             .iter()
//             .chain(chunk_type.iter())
//             .chain(message_bytes.iter())
//             .chain(crc.to_be_bytes().iter())
//             .copied()
//             .collect();

//         let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

//         let _chunk_string = format!("{}", chunk);
//     }
// }
