use chrono::prelude::*;
use failure::Error as FailureError;

use std::str::FromStr;

use super::evaluation::evaluate;
use super::{EntryId, Tags, UserId};
use error::AppError;

#[derive(Debug)]
pub struct Entry {
    pub id: EntryId,
    pub user_id: UserId,
    pub product: Product,
    pub time: NaiveDateTime,
    pub tags: Tags,
}

#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub price: i32,
}

impl Entry {
    pub fn new(user_id: UserId, product: Product) -> Entry {
        Entry {
            id: EntryId::generate(),
            user_id,
            product,
            time: ::chrono::offset::Local::now().naive_local(),
            tags: Tags::empty(),
        }
    }
}

impl FromStr for Product {
    type Err = FailureError;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let (price, name) = price_name(raw)?;

        Ok(Product {
            name: name.to_owned(),
            price: evaluate(price)?,
        })
    }
}

fn price_name(raw: &str) -> Result<(&str, &str), FailureError> {
    let raw = raw.trim();
    let mut price_first = None;
    let mut split_index = 0;
    for (index, ch) in raw.char_indices() {
        let price_part = match ch {
            '(' | ')' | ' ' | '*' | '-' | '+' | '/' | '0'...'9' => true,
            _ => false,
        };
        match (price_part, price_first) {
            (true, None) => price_first = Some(true),
            (true, Some(false)) => {
                split_index = index;
                break;
            }
            (true, Some(true)) => {}
            (false, None) => price_first = Some(false),
            (false, Some(false)) => {}
            (false, Some(true)) => {
                split_index = index;
                break;
            }
        }
    }

    let (price, name) = match price_first {
        None => return Err(AppError::Any {
            text: "В строке должны быть указаны продукт и цена",
        }.into()),
        Some(true) => raw.split_at(split_index),
        Some(false) => {
            let (name, price) = raw.split_at(split_index);
            (price, name)
        }
    };

    if name.is_empty() {
        return Err(AppError::Any {
            text: "В строке должны быть указаны продукт и цена",
        }.into());
    }

    Ok((price, name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_splits() {
        assert_eq!(price_name("чай 75"), Ok((" 75", "чай")));
    }
}
