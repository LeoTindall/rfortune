use rusqlite::{Connection, Error};
use quotes::Quote;

// SQL to create the quotes table in the database
//
const DATABASE_INIT_SQL: &'static str =
" CREATE TABLE quotes (
    id          INTEGER PRIMARY KEY,
    quote       TEXT NOT NULL,
    author      VARCHAR(255),
    source      TEXT
)";

const DATABASE_INSERT_QUOTE_SQL: &'static str =
" INSERT INTO quotes (quote, author, source) VALUES (?1, ?2, ?3)";

const DATABASE_QUERY_ALL_SQL: &'static str =
" SELECT * FROM quotes ";

pub fn get_database_connection(location: String) -> Result<Connection, Error> {
    let connection: Connection;
    if location == ":memory:" {
        connection = Connection::open_in_memory()?;
    } else {
        connection = Connection::open(location)?;
    }
    return Ok(connection);
}

pub fn initialize(connection: &mut Connection) -> Result<(), Error> {
    connection.execute(DATABASE_INIT_SQL, &[])?;
    Ok(())
}

pub fn add_quote(connection: &mut Connection, quote: &Quote) -> Result<(), Error> {
    connection.execute(DATABASE_INSERT_QUOTE_SQL, &[&quote.0, &quote.1, &quote.2])?;
    Ok(())
}

pub fn get_quotes(connection: &mut Connection) -> Result<Vec<Quote>, Error> {
    let mut statement = connection.prepare(DATABASE_QUERY_ALL_SQL)?;
    let mut maybe_quotes_iter = statement.query_map(&[], |row| {
        (row.get::<_, String>(1), row.get::<_, String>(2), row.get::<_, Option<String>>(3))
    })?;

    let mut quotes = Vec::new();
    for quote in maybe_quotes_iter {
        quotes.push(quote.unwrap());
    }

    return Ok(quotes)
}
