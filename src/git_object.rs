use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::fmt;
use std::io::Read;
use std::io::Write;
use std::str::FromStr;

pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectType::Blob => write!(f, "blob"),
            ObjectType::Tree => write!(f, "tree"),
            ObjectType::Commit => write!(f, "commit"),
        }
    }
}

impl FromStr for ObjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blob" => Ok(ObjectType::Blob),
            "tree" => Ok(ObjectType::Tree),
            "commit" => Ok(ObjectType::Commit),
            _ => Err(format!("unknown object type: {}", s)),
        }
    }
}

pub struct GitObject {
    object_type: ObjectType,
    content: Vec<u8>,
}

impl GitObject {
    pub fn new(object_type: ObjectType, content: Vec<u8>) -> Self {
        GitObject {
            object_type,
            content,
        }
    }

    pub fn hash(&self) -> String {
        let header = format!("{} {}\0", self.object_type, self.content.len());

        let mut hasher = Sha1::new();
        hasher.update(header.as_bytes());
        hasher.update(&self.content);

        format!("{:x}", hasher.finalize())
    }

    pub fn serialize(&self) -> Vec<u8> {
        let header = format!("{} {}\0", self.object_type, self.content.len());

        let mut data = Vec::new();
        data.extend_from_slice(header.as_bytes());
        data.extend_from_slice(&self.content);

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(&data)
            .expect("Encoder was not able to write properly");
        encoder
            .finish()
            .expect("Encoder was not able to finish properly")
    }

    pub fn deserialize(data: &[u8]) -> Self {
        // decompress
        let mut decoder = ZlibDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .expect("Not able to decompress");

        // find null byte index
        let null_idx = decompressed
            .iter()
            .position(|&b| b == b'\0')
            .expect("Not able to find the null byte value");

        // split header and content
        let header = &decompressed[..null_idx];
        let content = decompressed[null_idx + 1..].to_vec();

        // parse object type from header ("blob 11\0" -> "blob")
        let header_str = std::str::from_utf8(header).expect("Header string didn't found");
        let obj_type = header_str.split(' ').next().expect("Object type not found");

        Self {
            object_type: obj_type
                .parse::<ObjectType>()
                .expect("Can't parse the object type"),
            content,
        }
    }
}
