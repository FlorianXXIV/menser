use std::{fmt::Display, str::FromStr};

use ratatui::widgets::Row;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MensaMenu {
    pub menu: Menu,
    pub mensa_id: usize,
    pub mensa_name: String,
}

impl MensaMenu {
    pub fn to_table_rows(&self) -> Vec<Row> {
        let mut rows = Vec::new();
        for meal in &self.menu.meals {
            rows.push(Row::new([
                self.mensa_name.clone(),
                meal.name.clone(),
                meal.price.to_string(),
                meal.tags.to_string(),
                meal.time.clone(),
            ]));
        }

        rows
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Menu {
    pub meals: Vec<Meal>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Meal {
    pub id: usize,
    pub name: String,
    pub price: Price,
    pub tags: Tags,
    pub time: String,
}

impl Meal {
    pub fn is_lower_saxony_menu(&self) -> bool {
        self.tags.categories.contains(&Category {
            name: "Niedersachsen Menü".to_string(),
        })
    }

    pub fn ref_array(&self) -> [String; 5] {
        [
            self.id.to_string(),
            self.name.clone(),
            self.price.to_string(),
            self.tags.to_string(),
            self.time.clone(),
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Price {
    #[serde(deserialize_with = "price_deserialize")]
    pub student: u16,
}

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}€", self.student as f64 / 100.0)
    }
}

fn price_deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u16, D::Error> {
    let string = String::deserialize(deserializer)?;
    Ok((f64::from_str(&string).map_err(|e| serde::de::Error::custom(e))? * 100.0) as u16)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Tags {
    pub categories: Vec<Category>,
}

impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.categories
                .iter()
                .map(|c| c.name.clone() + " ")
                .collect::<String>()
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Category {
    pub name: String,
}
