use rusqlite::{Connection, Result};
use std::fs::create_dir_all;
use std::path::Path;

#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

fn get_conn() -> Result<Connection> {
    let dir = &shellexpand::tilde("~/.dbdir").into_owned();
    create_dir_all(dir).expect("Create storage dir");

    let path = &shellexpand::tilde("~/.dbdir/bookstore.db").into_owned();
    let path = Path::new(path);
    Connection::open(path)
}

pub fn init_storage() -> Result<()> {
    println!(" at ~/.dbdir/bookstore.db");
    let conn = get_conn()?;
    conn.execute(
        "Create table if not exists book_marks (
            id integer primary key,
            tag text not null unique,
            mark text not null unique
        )",
        [],
    )?;
    Ok(())
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

pub fn read_mark(tag: &str) -> Result<String> {
    let conn = get_conn()?;

    let mut stmt = conn.prepare(&format!(
        "SELECT mark from book_marks
        where tag = '{}';",
        &tag
    ))?;

    stmt.query_row([], |row| row.get(0))
}

pub fn add_mark(tag: &str, mark: &str) -> Result<()> {
    let conn = get_conn()?;

    conn.execute(
        "INSERT INTO book_marks (tag, mark) values (?1, ?2)",
        [&tag, &mark],
    )?;
    Ok(())
}

pub fn del_mark(tag: &str) -> Result<()> {
    let conn = get_conn()?;

    conn.execute("delete from book_marks where tag = ?1 ;", [&tag])?;

    Ok(())
}

pub fn update_mark(tag: &str, newtag: &str) -> Result<()> {
    let conn = get_conn()?;

    conn.execute(
        "update book_marks set tag = ?2 where tag = ?1;",
        [&tag, &newtag],
    )?;
    Ok(())
}
