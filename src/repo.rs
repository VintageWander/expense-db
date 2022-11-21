use mongodb::{bson::doc, Collection};

use crate::{model::Expense, mongo::DB, Result};

#[derive(Debug, Clone)]
pub struct ExpenseRepo {
    collection: Collection<Expense>,
}

impl ExpenseRepo {
    pub fn init(db: &DB) -> Self {
        Self {
            collection: db.get_collection("Expenses"),
        }
    }

    pub async fn create_expense(&self, expense: Expense) -> Result<Expense> {
        let new_id = self
            .collection
            .insert_one(expense, None)
            .await?
            .inserted_id
            .as_object_id()
            .ok_or("Cannot create a new expense")?;
        let expense = self
            .collection
            .find_one(doc! {"_id": new_id}, None)
            .await?
            .ok_or("Cannot find expense after create")?;
        Ok(expense)
    }

    pub async fn get_expense(&self, name: &str) -> Result<Expense> {
        let expense = self
            .collection
            .find_one(doc! {"username": name}, None)
            .await?
            .ok_or("Cannot find the expense")?;
        Ok(expense)
    }

    pub async fn delete_expense_by_name(&self, name: &str) -> Result<()> {
        self.collection
            .delete_one(doc! {"username": name}, None)
            .await?;
        Ok(())
    }

    pub async fn delete_all(&self) -> Result<()> {
        self.collection.delete_many(doc! {}, None).await?;
        Ok(())
    }
}
