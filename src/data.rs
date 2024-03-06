use std::io::{Cursor, Read, Write};

use serde::{de::DeserializeOwned, Serialize};

use crate::map::{Map, MapTile};

#[derive(Debug)]
pub enum DataError {
    BincodeSerialization(bincode::Error),
    IoError(std::io::Error),
}

impl From<bincode::Error> for DataError {
    fn from(value: bincode::Error) -> Self {
        DataError::BincodeSerialization(value)
    }
}


impl From<std::io::Error> for DataError {
    fn from(value: std::io::Error) -> Self {
        DataError::IoError(value)
    }
}

fn write_data<W, D>(buffer: &mut W, data: &D) -> Result<usize, DataError>
where
    W: Write,
    D: Serialize
{
    let bind = bincode::serialize(data)?;
    let nwrite = buffer.write(&bind)?;
    Ok(nwrite)
}

fn read_data<R, D>(buffer: &mut R) -> Result<D, DataError>
where
    R: Read,
    D: DeserializeOwned,
{
    Ok(bincode::deserialize_from(buffer)?)
}

#[test]
fn test_read_write_data() {
    use crate::game::GameEventInput;

    let data = GameEventInput::PlayerInput(3, "toto".into());
    let mut buffer = Vec::new();

    let res = write_data(&mut buffer, &data);
    assert!(res.is_ok());

    let mut buffer = Cursor::new(buffer);
    let data = read_data(&mut buffer);
    assert!(data.is_ok());
    let data : GameEventInput = data.unwrap();
}
