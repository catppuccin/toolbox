use std::collections::{BTreeMap, HashMap};

use crate::models::Color;

pub fn mix(args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let base: Color = tera::from_value(
        args.get("base")
            .ok_or_else(|| tera::Error::msg("base color is required"))?
            .clone(),
    )?;
    let blend: Color = tera::from_value(
        args.get("blend")
            .ok_or_else(|| tera::Error::msg("blend color is required"))?
            .clone(),
    )?;
    let amount = args
        .get("amount")
        .ok_or_else(|| tera::Error::msg("amount is required"))?
        .as_f64()
        .ok_or_else(|| tera::Error::msg("amount must be a number"))?;

    let result = Color::mix(&base, &blend, amount);

    Ok(tera::to_value(result)?)
}

pub fn if_fn(args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let cond = args
        .get("cond")
        .ok_or_else(|| tera::Error::msg("cond is required"))?
        .as_bool()
        .ok_or_else(|| tera::Error::msg("cond must be a boolean"))?;
    let t = args
        .get("t")
        .ok_or_else(|| tera::Error::msg("t is required"))?
        .clone();
    let f = args
        .get("f")
        .ok_or_else(|| tera::Error::msg("f is required"))?
        .clone();

    Ok(if cond { t } else { f })
}

pub fn object(args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    // sorting the args gives us stable output
    let args: BTreeMap<_, _> = args.iter().collect();
    Ok(tera::to_value(args)?)
}
