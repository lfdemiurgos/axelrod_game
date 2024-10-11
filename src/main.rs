use std::{env, error, fs, io, path::PathBuf, result};

use rusqlite::Connection;

type Result<T> = result::Result<T, Box<dyn error::Error>>;

const DEFAULT_PATH: &str = "./database/axelrod.db";

fn setup() -> Result<Connection> {
    let env_var = env::var("DATABASE_PATH").unwrap_or_else(|_|{
        println!("Warning: The `DATABASE_PATH` environment variable is not set, {DEFAULT_PATH} will be used instead.");
        DEFAULT_PATH.into()
    });
    let db_path = PathBuf::from(env_var);
    let db_dir = &db_path.parent().ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Error: Invalid path to database",
    ))?;
    if !&db_path.exists() {
        fs::create_dir_all(db_dir)?;
        println!(
            "Warning: database at {} does not exists, will be created.",
            db_path.display()
        );
    }
    let conn = Connection::open(&db_path)?;
    Ok(conn)
}

fn main() -> Result<()> {
    setup()?;
    Ok(())
}
