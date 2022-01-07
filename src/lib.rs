use std::path::Path;
use rusqlite::{Connection, Result};

#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

fn get_conn() -> Result<Connection> {
    let path = &shellexpand::tilde("~/.dbdir/bookstore.db").into_owned();
    let path = Path::new(path);
    Connection::open(path)
}
pub fn read_tags() -> Result<Vec<String>> {
    let conn = get_conn()?;
    let mut stmt = conn.prepare("SELECT tag from book_marks ;")?;
    let rows = stmt.query_map([], |row| Ok(row.get(0)))?;
    let mut tags = Vec::new();
    for row in rows {
        tags.push(row.unwrap()?);
    }
    Ok(tags)
}