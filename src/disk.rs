use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub tasks: Vec<Task>,
}

pub fn load() -> io::Result<Data> {
    let mut file = match File::open(make_path()) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Ok(Data { tasks: Vec::new() });
        }
        Err(e) => return Err(e),
    };
    let data = serde_json::from_reader(&mut file)?;
    Ok(data)
}

pub fn save(data: &Data) -> io::Result<()> {
    let mut file = File::create(make_path())?;
    serde_json::to_writer(&mut file, &data)?;
    Ok(())
}

fn make_path() -> PathBuf {
    dirs::data_dir().unwrap().join("todo-thingy.json")
}
