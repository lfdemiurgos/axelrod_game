use std::{env, error, result};

const ENV_VARIABLE: &str = "DATABASE_PATH";
const DEFAULT_PATH: &str = "./database/axelrod.db";

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub fn get_var() -> Result<String> {
    env::var(ENV_VARIABLE).or_else(|_|{
        println!("Warning: The `DATABASE_PATH` environment variable is not set, {DEFAULT_PATH} will be used instead.");
        Ok(DEFAULT_PATH.into())
    })
}
