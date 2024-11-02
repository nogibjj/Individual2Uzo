#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use rusqlite::params;
    use rusqlite::Connection;
    use rust_cli_binary::{create, delete, extract, read, update};
    use std::fs;
    use std::sync::Once;

    lazy_static! { 
        static ref INIT: Once = Once::new();
    }

    #[test]
    fn test_extract() {
        let test_url = "https://github.com/fivethirtyeight/data/raw/refs/heads/master/unisex-names/unisex_names_table.csv";
        let test_path = "test_unisex_names.csv";
        
        let result = extract(test_url, test_path);
        
        assert!(result.is_ok(), "Extract function failed with {:?}", result);
        assert!(
            fs::metadata(test_path).is_ok(),
            "Failed to create the file at {}",
            test_path
        );
    }

    #[test]
    fn test_create() {
        let db_path = "test_unisexDB.db";

        // Clear the table at the beginning of the test
        let conn = Connection::open(db_path).unwrap();
        conn.execute("DELETE FROM unisex_names", []).unwrap();

        let result = create(db_path, 7, "Alex", 3000, 0.45, 0.55, 0.1);
        assert!(result.is_ok(), "Create function failed with {:?}", result);

        let total: i32 = conn
            .query_row(
                "SELECT total FROM unisex_names WHERE id = 7 AND name = 'Alex'",
                params![],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(total, 3000);
    }

    #[test]
    fn test_read() {
        let db_path = "test_unisexDB.db";
        let conn = Connection::open(db_path).unwrap();

        // Clear the table at the beginning of the test
        conn.execute("DELETE FROM unisex_names", []).unwrap();

        conn.execute(
            "INSERT INTO unisex_names (id, name, total, male_share, female_share, gap) VALUES (8, 'Taylor', 5000, 0.45, 0.55, 0.1)",
            [],
        )
        .unwrap();

        let result = read(db_path);
        assert!(result.is_ok(), "Read function failed with {:?}", result);
    }

    #[test]
    fn test_update() {
        let db_path = "test_unisexDB.db";
        let conn = Connection::open(db_path).unwrap();

        // Clear the table at the beginning of the test
        conn.execute("DELETE FROM unisex_names", []).unwrap();

        conn.execute(
            "INSERT INTO unisex_names (id, name, total, male_share, female_share, gap) VALUES (9, 'Jordan', 4000, 0.50, 0.50, 0.0)",
            [],
        )
        .unwrap();

        let result = update(db_path, 9, "Jordan", 4500, 0.50, 0.50, 0.0);
        assert!(result.is_ok(), "Update function failed with {:?}", result);

        let total: i32 = conn
            .query_row(
                "SELECT total FROM unisex_names WHERE id = 9 AND name = 'Jordan'",
                params![],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(total, 4500);
    }

    #[test]
    fn test_delete() {
        let db_path = "test_unisexDB.db";
        let conn = Connection::open(db_path).unwrap();

        // Clear the table at the beginning of the test
        conn.execute("DELETE FROM unisex_names", []).unwrap();

        conn.execute(
            "INSERT INTO unisex_names (id, name, total, male_share, female_share, gap) VALUES (10, 'Chris', 6000, 0.40, 0.60, 0.2)",
            [],
        )
        .unwrap();

        let result = delete(db_path, 10);
        assert!(result.is_ok(), "Delete function failed with {:?}", result);

        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM unisex_names WHERE id = 10",
                params![],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0, "Expected no records with id = 10, found {}", count);
    }
}
