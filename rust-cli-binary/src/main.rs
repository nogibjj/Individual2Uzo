use rust_cli_binary::{extract, transform, create, read, update, delete};
use std::fs;

fn main() {
    // Remove the existing database file if it exists
    let _ = fs::remove_file("unisexDB.db");

    // Extract data from the specified URL to a CSV file
    if let Err(e) = extract(
        "https://github.com/fivethirtyeight/data/raw/refs/heads/master/unisex-names/unisex_names_table.csv",
        "unisex_names_table.csv",
    ) {
        eprintln!("Failed to extract data: {}", e);
        return;
    }

    let csv_path = "unisex_names_table.csv";
    let db_path = "unisexDB.db";

    // Transform CSV data into an SQLite database
    match transform(csv_path, db_path) {
        Ok(_) => println!("CSV file has been successfully converted to SQLite DB."),
        Err(e) => println!("An error occurred during conversion: {}", e),
    }

    // Create a new record in the database
    match create("unisexDB.db", 7, "Alex", 3000, 0.45, 0.55, 0.1) {
        Ok(_) => println!("Successfully inserted data into the SQLite DB."),
        Err(e) => println!("Error occurred while inserting data: {}", e),
    }

    // Read and display records from the database
    match read("unisexDB.db") {
        Ok(_) => println!("Successfully read from the SQLite DB."),
        Err(e) => println!("Error occurred while reading data: {}", e),
    }

    println!();

    // Update a record in the database
    match update("unisexDB.db", 8, "Taylor", 5000, 0.45, 0.55, 0.1) {
        Ok(_) => println!("Successfully updated data in the SQLite DB."),
        Err(e) => println!("Error occurred while updating data: {}", e),
    }

    // Read and display records after update
    match read("unisexDB.db") {
        Ok(_) => println!("Successfully read from the SQLite DB."),
        Err(e) => println!("Error occurred while reading data: {}", e),
    }

    println!();

    // Delete a record by ID
    let id_to_delete = 9;
    match delete(db_path, id_to_delete) {
        Ok(_) => println!("Successfully deleted data for ID {}.", id_to_delete),
        Err(e) => println!("An error occurred: {}", e),
    }

    // Final read to confirm deletion
    match read("unisexDB.db") {
        Ok(_) => println!("Successfully read from the SQLite DB."),
        Err(e) => println!("Error occurred while reading data: {}", e),
    }

    println!();
}
