mod app;
mod chat_room;
mod crossterm;
mod topic_list;
mod ui;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    crossterm::run()?;
    Ok(())
}
