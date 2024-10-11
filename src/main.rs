use std::{
    env, error, fs, io,
    path::{Path, PathBuf},
    result,
};

use rusqlite::Connection;

type Result<T> = result::Result<T, Box<dyn error::Error>>;

const ENV_VARIABLE: &str = "DATABASE_PATH";
const DEFAULT_PATH: &str = "./database/axelrod.db";

fn get_var() -> Result<String> {
    env::var(ENV_VARIABLE).or_else(|_|{
        println!("Warning: The `DATABASE_PATH` environment variable is not set, {DEFAULT_PATH} will be used instead.");
        Ok(DEFAULT_PATH.into())
    })
}

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

fn setup() -> Result<Connection> {
    let path = get_var().map(PathBuf::from)?;
    if !&path.exists() {
        create_at(&path)
    } else {
        validate(&path)
    }
}

fn main() -> Result<()> {
    setup()?;
    Ok(())
}
