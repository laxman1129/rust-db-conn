use sqlx::Row;
use std::error::Error;

struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn get_book(isbn: &str, pool: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
    let query = "SELECT title, author, isbn FROM book WHERE isbn = $1";
    let res = sqlx::query(query).bind(isbn).fetch_one(pool).await?;

    Ok(Book {
        title: res.get("title"),
        author: res.get("author"),
        isbn: res.get("isbn"),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:postgres@localhost:5432/test_db";
    // let mut conn = sqlx::postgres::PgConnection::connect(url).await?; // use sqlx::{Connection};
    let pool = sqlx::postgres::PgPool::connect(url).await?; // for connection pooling

    // call migrate macro to run migrations
    // sqlx::migrate!("./migrations").run(&pool).await?;

    let res = sqlx::query("select 1+1 as sum")
        // .fetch_one(&mut conn) // for single connection
        .fetch_one(&pool)
        .await?;

    let sum: i32 = res.get("sum");
    println!("The sum is: {}", sum);

    let book = Book {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
        isbn: "978-1593278282".to_string(),
    };

    create(&book, &pool).await?;
    
    let fetched_book = get_book(&book.isbn, &pool).await?;
    println!(
        "Fetched Book: Title: {}, Author: {}, ISBN: {}",
        fetched_book.title, fetched_book.author, fetched_book.isbn
    );

    Ok(())
}
