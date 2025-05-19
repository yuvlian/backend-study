use crate::PeakResult;
use chrono::{DateTime, Datelike, Local};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, FromRow)]
pub struct ExpenseTable {
    pub id: i64,
    pub date: DateTime<Local>,
    pub description: String,
    pub amount: i64,
}

impl ExpenseTable {
    pub async fn create_if_not_exists(pool: &SqlitePool) -> PeakResult<()> {
        const QUERY: &str = r#"create table if not exists expense (
            id integer primary key,
            date text not null,
            description text not null,
            amount integer not null
        )"#;

        sqlx::query(QUERY).execute(pool).await?;
        Ok(())
    }

    pub async fn insert_new_expense(
        pool: &SqlitePool,
        description: &str,
        amount: i64,
    ) -> PeakResult<i64> {
        const QUERY: &str = "insert into expense (date, description, amount) values (?, ?, ?)";

        let id = sqlx::query(QUERY)
            .bind(Local::now())
            .bind(description)
            .bind(amount)
            .execute(pool)
            .await?
            .last_insert_rowid();
        Ok(id)
    }

    pub async fn fetch_all_expense(pool: &SqlitePool) -> PeakResult<Vec<Self>> {
        const QUERY: &str = "select * from expense";
        Ok(sqlx::query_as(QUERY).fetch_all(pool).await?)
    }

    pub async fn delete_expense_by_id(pool: &SqlitePool, id: i64) -> PeakResult<u64> {
        const QUERY: &str = "delete from expense where id = ?";

        let result = sqlx::query(QUERY)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();
        Ok(result)
    }

    pub async fn sum_expenses(pool: &SqlitePool) -> PeakResult<i64> {
        const QUERY: &str = "select sum(amount) from expense";

        let total: (Option<i64>,) = sqlx::query_as(QUERY).fetch_one(pool).await?;

        Ok(total.0.unwrap_or_default())
    }

    pub async fn sum_expenses_by_month_num(pool: &SqlitePool, month: u32) -> PeakResult<i64> {
        let expense_rows = Self::fetch_all_expense(pool).await?;
        let mut total = 0;
        for r in expense_rows {
            if r.date.month() == month {
                total += r.amount;
            }
        }
        Ok(total)
    }
}
