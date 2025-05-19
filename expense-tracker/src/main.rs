use clap::Parser;
use sqlx::SqlitePool;
use std::path::Path;
use tokio::fs;

mod args;
mod table;

use args::{Args, Commands};
use table::ExpenseTable;

// too lazy to do proper errors
type ErrorSS = Box<dyn std::error::Error + Send + Sync>;
type PeakResult<T> = Result<T, ErrorSS>;

const DB_FILE_BYTES: &[u8] = include_bytes!("../sqlite.db");

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let new_db = !Path::new("./sqlite.db").exists();

    if new_db {
        fs::write("./sqlite.db", DB_FILE_BYTES).await.unwrap();
    }

    let pool = SqlitePool::connect("sqlite:sqlite.db").await.unwrap();

    if new_db {
        ExpenseTable::create_if_not_exists(&pool).await.unwrap();
    }

    let args = Args::parse();

    match args.command {
        Commands::Add {
            description,
            amount,
        } => match ExpenseTable::insert_new_expense(&pool, &description, amount).await {
            Ok(v) => println!("Expense added successfully (ID: {})", v),
            Err(e) => eprintln!("Error adding expense: {}", e),
        },
        Commands::List => match ExpenseTable::fetch_all_expense(&pool).await {
            Ok(expenses) => {
                println!("[ID]--[Date]--[Description]--[Amount]\n");
                for ex in expenses {
                    println!(
                        "[{}]--[{}]--[{}]--[${}]",
                        ex.id,
                        ex.date.to_string().split(".").nth(0).unwrap(),
                        ex.description,
                        ex.amount
                    );
                }
            }
            Err(e) => eprintln!("Error listing expenses: {}", e),
        },
        Commands::Summary { month: Some(month) } => {
            match ExpenseTable::sum_expenses_by_month_num(&pool, month).await {
                Ok(v) => println!("Total expense: ${}", v),
                Err(e) => eprintln!("Error summing expenses: {}", e),
            }
        }
        Commands::Summary { month: None } => match ExpenseTable::sum_expenses(&pool).await {
            Ok(v) => println!("Total expense: ${}", v),
            Err(e) => eprintln!("Error summing expenses: {}", e),
        },
        Commands::Delete { id } => match ExpenseTable::delete_expense_by_id(&pool, id).await {
            Ok(1..) => println!("Expense with id {} deleted successfully", id),
            Ok(0) => println!("Couldn't find expense with id {}", id),
            Err(e) => eprintln!("Error deleting expense: {}", e),
        },
    }
}
