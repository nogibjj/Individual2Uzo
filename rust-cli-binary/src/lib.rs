use rusqlite::{params, Connection, Result};
use std::fs::File;
use std::io::Write;
use reqwest;

// Extracts dataset from a URL and saves it to a specified file path
pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch data from the URL via an HTTP request
    let response = reqwest::blocking::get(url)?;

    // Read the content of the fetched data
    let content = response.text()?;

    // Create a file at the specified path and write the content
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

// Transforms CSV data and loads it into an SQLite database
pub fn transform(csv_path: &str, db_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the CSV file
    let mut rdr = csv::Reader::from_path(csv_path)?;

    // Create or connect to an SQLite database file
    let conn = Connection::open(db_path)?;

    // Create the appropriate table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS unisex_names (
            id INTEGER PRIMARY KEY,
            name TEXT,
            total INTEGER,
            male_share REAL,
            female_share REAL,
            gap REAL
        )",
        [],
    )?;

    // Insert data from the CSV into the table
    for result in rdr.records() {
        let record = result?;
        let id: i32 = record[0].parse()?;
        let name: String = record[1].to_string();
        let total: i32 = record[2].parse()?;
        let male_share: f64 = record[3].parse()?;
        let female_share: f64 = record[4].parse()?;
        let gap: f64 = record[5].parse()?;

        conn.execute(
            "INSERT INTO unisex_names (id, name, total, male_share, female_share, gap) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, name, total, male_share, female_share, gap],
        )?;
    }

    Ok(())
}

// Creates a new record in the database
pub fn create(
    db_path: &str,
    id: i32,
    name: &str,
    total: i32,
    male_share: f64,
    female_share: f64,
    gap: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "INSERT INTO unisex_names (id, name, total, male_share, female_share, gap) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, name, total, male_share, female_share, gap],
    )?;

    println!("Data successfully created in the database.");
    Ok(())
}

// Reads all records from the database and prints them
pub fn read(db_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the SQLite database file
    let conn = Connection::open(db_path)?;

    // Execute the query and fetch the results
    let mut stmt = conn.prepare("SELECT id, name, total, male_share, female_share, gap FROM unisex_names")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i32>(2)?,
            row.get::<_, f64>(3)?,
            row.get::<_, f64>(4)?,
            row.get::<_, f64>(5)?,
        ))
    })?;

    // Print the results
    for row_result in rows {
        let (id, name, total, male_share, female_share, gap) = row_result?;
        println!("{}, {}, {}, {}, {}, {}", id, name, total, male_share, female_share, gap);
    }

    Ok(())
}

// Updates a record in the database for a specific unisex name
pub fn update(
    db_path: &str,
    id: i32,
    name: &str,
    total: i32,
    male_share: f64,
    female_share: f64,
    gap: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(db_path)?;

    let rows_modified = conn.execute(
        "UPDATE unisex_names SET name = ?2, total = ?3, male_share = ?4, female_share = ?5, gap = ?6 WHERE id = ?1",
        params![id, name, total, male_share, female_share, gap],
    )?;

    if rows_modified == 0 {
        println!("No data found for the specified ID.");
    } else {
        println!("Data successfully updated in the database.");
    }
    Ok(())
}

// Deletes records from the database for a specified id
pub fn delete(db_path: &str, id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(db_path)?;

    // Delete records for the specified id
    let rows_deleted = conn.execute("DELETE FROM unisex_names WHERE id = ?", params![id])?;
    
    if rows_deleted == 0 {
        println!("No records found for the specified ID.");
    } else {
        println!("Data successfully deleted from the database.");
    }

    Ok(())
}
