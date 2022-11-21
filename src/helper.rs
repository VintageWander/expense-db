use salvo::Depot;

use salvo::Request;
use serde::Deserialize;

use crate::repo::ExpenseRepo;
use crate::Result;

pub fn extract_from_depot<'a, T: Sync + Send + Clone + 'static>(
    depot: &'a Depot,
    key: &'a str,
) -> Result<&'a T> {
    depot
        .get::<T>(key)
        .ok_or_else(|| format!("Cannot get {} from depot", key).into())
}
pub fn get_expenses_repo(depot: &Depot) -> Result<&ExpenseRepo> {
    extract_from_depot(depot, "expenses_repo")
}

pub async fn extract_from_body<'a, T: Deserialize<'a>>(req: &'a mut Request) -> Result<T> {
    Ok(req.parse_body::<T>().await?)
}
