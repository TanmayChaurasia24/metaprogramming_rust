use std::{fmt::Error, result};

trait Serialize {
	fn serialize(&self) -> Vec<u8>;
}

trait Deserialize: Sized {
	fn deserialize(base: &[u8]) -> Result<Self, Error>;
}

#[derive(Debug)]
struct swap {
    qnt_1: i32,
    qnt_2: i32
}

impl Serialize for swap {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(&self.qnt_1.to_be_bytes());
        result.extend_from_slice(&self.qnt_2.to_be_bytes());
        result
    }
}

impl Deserialize for swap {
    fn deserialize(base: &[u8]) -> Result<Self, Error> {
        if base.len() < 8 {
            return Err(Error);
        }

        let qty_1_bytes: [u8; 4] = base[0..4].try_into().map_err(|_| Error)?;
        let qnt_1 = i32::from_be_bytes(qty_1_bytes);
        
        let qty_2_bytes: [u8; 4] = base[4..8].try_into().map_err(|_| Error)?;
        let qnt_2 = i32::from_be_bytes(qty_2_bytes);

        Ok(swap{qnt_1,qnt_2})
    }
}

fn main() {
    let s = swap {
        qnt_1: 1,
        qnt_2: 2
    };

    let bytes = s.serialize();
    let s2 = swap::deserialize(&bytes).unwrap();
    print!("{:?}", bytes);
    assert!(s.qnt_1 == s2.qnt_1);
    assert!(s.qnt_2 == s2.qnt_2);
    println!("Test done")

}