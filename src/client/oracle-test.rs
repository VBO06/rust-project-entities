pub mod oracle {
    use oracle::{Connection, Result};
    // Select a table and print column types and values as CSV.
    // The CSV format isn't valid if data include double quotation
    // marks, commas or return codes.
    pub fn connect() -> Result<()> {
        let username = "XXXX";
        let password = "XXXX";
        let database = "XXXX";
        let sql = "select * from XXXX";

        let conn = Connection::connect(username, password, database)?;
        let mut stmt = conn.statement(sql).build()?;
        let rows = stmt.query(&[])?;

        // print column types
        for (idx, info) in rows.column_info().iter().enumerate() {
            if idx != 0 {
                print!(",");
            }
            print!("{}", info);
        }
        println!();

        for row_result in rows {
            // print column values
            for (idx, val) in row_result?.sql_values().iter().enumerate() {
                if idx != 0 {
                    print!(",");
                }
                print!("{}", val);
            }
            println!();
        }
        Ok(())
    }
}