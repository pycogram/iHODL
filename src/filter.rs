use crate::types::TokenHolder;

/// Filter holders by minimum UI amount (considering decimals)
pub fn filter_by_minimum_ui_amount(
    holders: Vec<TokenHolder>,
    minimum_ui_amount: f64,
) -> Vec<TokenHolder> {
    holders
        .into_iter()
        .filter(|holder| holder.get_ui_amount() >= minimum_ui_amount)
        .collect()
}

/// Sort holders by balance in descending order (highest first)
pub fn sort_by_balance_desc(mut holders: Vec<TokenHolder>) -> Vec<TokenHolder> {
    holders.sort_by(|a, b| b.balance.cmp(&a.balance));
    holders
}
