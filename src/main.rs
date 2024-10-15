mod db;
mod models;
mod utils;

fn main() -> utils::Result<()> {
    db::setup()?;
    Ok(())
}
