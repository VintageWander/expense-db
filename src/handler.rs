use salvo::{handler, Depot, Request, Response};
use validator::Validate;

use crate::{
    helper::{extract_from_body, get_expenses_repo},
    model::Expense,
    web::Web,
    WebResult,
};

#[handler]
pub async fn create_trips_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> WebResult {
    let expense_req = extract_from_body::<Expense>(req).await?;

    expense_req.validate()?;

    let expenses_repo = get_expenses_repo(depot)?;

    expenses_repo.delete_all().await?;
    let new_expense = expenses_repo.create_expense(expense_req).await?;

    Ok(Web::ok("Expense posted successfully", new_expense))
}

#[handler]
pub async fn hello() -> WebResult {
    Ok(Web::ok("Connect success", ()))
}
