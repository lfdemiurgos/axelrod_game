use std::{
    fs, io,
    path::{Path, PathBuf},
};

use rusqlite::Connection;

use crate::utils::get_var;
use crate::utils::Result;

fn create_at(path: &Path) -> Result<Connection> {
    println!(
        "Warning: database at {} does not exists, will be created.",
        path.display()
    );
    let dir = path
        .parent()
        .ok_or(io::Error::from(io::ErrorKind::InvalidInput))?;
    fs::create_dir_all(dir)?;
    let conn = Connection::open(path)?;
    conn.execute("VACUUM;", ())?;
    Ok(conn)
}

fn validate(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.pragma(None, "integrity_check", 1, |_| Ok(()))?;
    Ok(conn)
}

pub fn setup() -> Result<Connection> {
    let path = get_var().map(PathBuf::from)?;
    if !&path.exists() {
        create_at(&path)
    } else {
        validate(&path)
    }
}
