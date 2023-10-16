use std::{collections::BTreeMap, fs::File, path::Path};

use float_eq::assert_float_eq;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema)]
struct Item {
    price: f32,
    people: BTreeMap<String, f32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct Receipt {
    items: BTreeMap<String, Item>,
    extras: BTreeMap<String, f32>,
    total: f32,
}

#[derive(Debug)]
struct MoneyOutput(f32);

impl Serialize for MoneyOutput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut string = format!("{:.2}", self.0);
        if string.starts_with('.') {
            string.insert(0, '0');
        }
        serializer.serialize_str(&string)
    }
}

#[derive(Debug, Serialize)]
struct Output {
    item_costs: BTreeMap<String, MoneyOutput>,
    person_shares: BTreeMap<String, MoneyOutput>,
}

impl Receipt {
    fn get_item_costs(&self) -> BTreeMap<String, f32> {
        // Get the total cost of all extra
        let total_extras: f32 = self.extras.values().sum();

        // Get the total cost of all items
        let total_items: f32 = self.items.values().map(|item| item.price).sum();

        // Add the weighted cost of total extras to each item
        self.items
            .iter()
            .map(|(name, item)| {
                (
                    name.clone(),
                    item.price + item.price * total_extras / total_items,
                )
            })
            .collect()
    }

    fn get_person_shares(&self) -> BTreeMap<String, f32> {
        let mut shares = BTreeMap::new();
        let item_costs = self.get_item_costs();
        for (name, item) in &self.items {
            // Take the sum of the weights of all people who shared this item
            // The quotient of a person's weight and the sum of all weights is their share
            let sum: f32 = item.people.values().sum();
            for (person, weight) in &item.people {
                let share = weight / sum * item_costs[name];
                *shares.entry(person.clone()).or_insert(0.0) += share;
            }
        }
        shares
    }
}

// Load a receipt from a JSON file and calculate item costs and person shares
// This function should accept a path which satisfies the AsPath trait
pub fn process_receipt<P: AsRef<Path>>(
    input_path: P,
    output_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let receipt: Receipt = serde_json::from_reader(file)?;
    let item_costs = receipt.get_item_costs();
    let person_shares = receipt.get_person_shares();
    assert_float_eq!(item_costs.values().sum::<f32>(), receipt.total, abs <= 0.01);
    assert_float_eq!(
        person_shares.values().sum::<f32>(),
        receipt.total,
        abs <= 0.01
    );
    let output = Output {
        item_costs: item_costs
            .into_iter()
            .map(|(k, v)| (k, MoneyOutput(v)))
            .collect(),
        person_shares: person_shares
            .into_iter()
            .map(|(k, v)| (k, MoneyOutput(v)))
            .collect(),
    };
    let file = File::create(output_path)?;
    serde_json::to_writer_pretty(file, &output)?;
    Ok(())
}

pub fn print_schema<P: AsRef<Path>>(output_path: P) -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_for!(Receipt);
    let file = File::create(output_path)?;
    serde_json::to_writer_pretty(file, &schema)?;
    Ok(())
}
