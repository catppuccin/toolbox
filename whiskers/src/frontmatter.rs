use handlebars::Handlebars;
use json_patch::merge;
use serde_json::Value;

use crate::{Map, COLOR_NAMES};

pub type FlavorContexts = Vec<Option<Value>>;

fn split(template: &str) -> Option<(&str, &str)> {
    // we consider a template to possibly have frontmatter iff:
    // * line 0 is "---"
    // * there is another "---" on another line
    let template = template.trim();
    let sep = "---";
    if !template.starts_with(sep) {
        return None;
    }

    template[sep.len()..]
        .split_once(sep)
        .map(|(a, b)| (a.trim(), b.trim()))
}

/// Merges together overrides from the cli and frontmatter.
///
/// The order is as follows:
///
/// 1. CLI overrides from the `--overrides` flag.
/// 2. The `"overrides": "all"` frontmatter block.
/// 3. The `"overrides": ("latte" | "frappe" | "macchiato" | "mocha")` frontmatter block(s)
///
fn merge_overrides(
    cli_overrides: Option<Value>,
    frontmatter: Option<Value>,
    flavor: &str,
) -> String {
    if let Some(fm) = frontmatter {
        // applying cli overrides
        let mut merged = cli_overrides.map_or(fm.clone(), |cli| {
            let mut merged = fm;
            merge(&mut merged, &cli);
            merged
        });

        if let Some(yaml) = merged.get("overrides").cloned() {
            // hoisting "all" overrides to root context
            if let Some(all) = yaml.get("all") {
                merge(&mut merged, all);
            }
            // hosting current flavor overrides to root context
            if let Some(flavor) = yaml.get(flavor) {
                merge(&mut merged, flavor);
            }
        }

        let merged_mut = merged
            .as_object_mut()
            .expect("merged can be converted to a map");

        // Don't need the "overrides" block anymore since we've hoisted everything up
        // and therefore removing it, but we could keep it in.
        merged_mut.remove("overrides");

        // I suppose I also have to do the ugly thing of checking if the variable
        // is a colour from our palette so that we can also insert it into the
        // ["colors"] handlebars iterator?
        let colours = merged_mut
            .clone()
            .into_iter()
            .filter(|(k, _)| COLOR_NAMES.iter().any(|s| s == k))
            .collect::<Map>();
        if !colours.is_empty() {
            merged_mut.insert("colors".to_string(), Value::from(colours));
        }

        // Also, this code isn't performing any validation to check if the override
        // variables exist beforehand, suppose we need to decide if that's a feature
        // or bug?

        serde_json::to_string(merged_mut).expect("merged can be serialized to a string")
    } else {
        cli_overrides
            .expect("cli overrides can be serialized to a string")
            .to_string()
    }
}

#[must_use]
#[allow(clippy::missing_panics_doc)] // panic here implies an internal issue
pub fn render_and_parse_all<'a>(
    template: &'a str,
    overrides: &Option<Value>,
    reg: &Handlebars,
    ctx: &Value,
) -> (&'a str, Map) {
    let Some((_, content)) = split(template) else {
        return (
            template,
            [
                ("latte".to_string(), Value::Null),
                ("frappe".to_string(), Value::Null),
                ("macchiato".to_string(), Value::Null),
                ("mocha".to_string(), Value::Null),
            ]
            .iter()
            .cloned()
            .collect::<Map>(),
        );
    };

    let frontmatter = ctx
        .as_object()
        .expect("context is an object")
        .into_iter()
        .map(|(flavor, ctx)| {
            let frontmatter = render_and_parse(template, overrides.clone(), flavor, reg, ctx).1;
            (flavor.to_string(), frontmatter)
        })
        .collect::<Map>();

    (content, frontmatter)
}

#[must_use]
pub fn render_and_parse<'a>(
    template: &'a str,
    overrides: Option<Value>,
    flavor: &'a str,
    reg: &Handlebars,
    ctx: &Value,
) -> (&'a str, Value) {
    let Some((frontmatter, content)) = split(template) else {
        return (template, Value::Null);
    };

    let frontmatter_parsed = match serde_yaml::from_str(frontmatter) {
        Ok(frontmatter) => frontmatter,
        Err(e) => {
            eprintln!("warning: Failed to parse YAML frontmatter ({e}). Proceeding without it.");
            return (content, Value::Null);
        }
    };

    let frontmatter_with_overrides = merge_overrides(overrides, frontmatter_parsed, flavor);

    let frontmatter_rendered = match reg.render_template(&frontmatter_with_overrides, ctx) {
        Ok(frontmatter) => frontmatter,
        Err(e) => {
            eprintln!(
                "warning: Failed to render frontmatter templates ({e}). Proceeding without it"
            );
            return (content, Value::Null);
        }
    };

    match serde_yaml::from_str(&frontmatter_rendered) {
        Ok(frontmatter) => (content, frontmatter),
        Err(e) => {
            eprintln!("warning: Failed to parse YAML frontmatter ({e}). Proceeding without it.");
            (content, Value::Null)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Map;

    use super::*;

    #[test]
    fn no_frontmatter() {
        let content = "a\nb\nc";
        let result = split(content);
        assert_eq!(result, None);
    }

    #[test]
    fn unclosed_frontmatter() {
        let content = "---\na: b\nc: d";
        let result = split(content);
        assert_eq!(result, None);
    }

    #[test]
    fn all_frontmatter_no_template() {
        let content = "---\na: b\nc: d\n---";
        let result = split(content);
        assert_eq!(result, Some(("a: b\nc: d", "")));
    }

    #[test]
    fn some_frontmatter_some_template() {
        let content = "---\na: b\nc: d\n---\na: b\nc: d\n";
        let result = split(content);
        assert_eq!(result, Some(("a: b\nc: d", "a: b\nc: d")));
    }

    #[test]
    fn parse_frontmatter() {
        let content = "---\na: b\nc: d\n---\na: b\nc: d\n";
        let expected =
            serde_json::from_str::<Value>(r#"{"a":"b","c":"d"}"#).expect("valid json fixture");
        let reg = Handlebars::new();
        let ctx = Value::Object(Map::new());
        let overrides = Some(Value::Object(Map::new()));
        let result = render_and_parse(content, overrides, &reg, &ctx);
        assert_eq!(result, ("a: b\nc: d", Some(expected)));
    }

    // #[test]
    // fn parse_root_context_frontmatter_when_flavor_all() {
    //     let content = "---\na: b\nc: d\n---\na: b\nc: d\n";
    //     let expected =
    //         serde_json::from_str::<Map<_, _>>(r#"{"a":"b","c":"d"}"#).expect("valid json fixture");
    //     let reg = Handlebars::new();
    //     let ctx = Value::Object(serde_json::Map::new());
    //     let result = render_and_parse_all(content, &reg, &ctx);
    //     assert_eq!(result, ("a: b\nc: d", vec![], Some(expected)));
    // }
    //
    #[test]
    fn render_frontmatter() {
        let content = "---\na: {{var}}\nc: d\n---\na: b\nc: d\n";
        let expected =
            serde_json::from_str::<Value>(r#"{"a":"b","c":"d"}"#).expect("valid json fixture");
        let reg = Handlebars::new();
        let ctx = serde_json::from_str::<Value>(r#"{"var":"b"}"#).expect("valid json fixture");
        let overrides = Some(Value::Object(Map::new()));
        let result = render_and_parse(content, overrides, &reg, &ctx);
        assert_eq!(result, ("a: b\nc: d", Some(expected)));
    }

    // #[test]
    // fn render_frontmatter_when_flavor_all() {
    //     let content = "---\na: '{{num}}'\nc: d\n---\na: b\nc: d\n";
    //     let expected: Vec<Option<Value>> = vec![
    //         Some(serde_json::json!({"a": "1"})),
    //         Some(serde_json::json!({"a": "2"})),
    //         Some(serde_json::json!({"a": "3"})),
    //         Some(serde_json::json!({"a": "4"})),
    //     ];
    //     let expected_root: Option<Map<String, Value>> =
    //         serde_json::json!({"c": "d"}).as_object().cloned();
    //     let reg = Handlebars::new();
    //     let ctx = serde_json::from_str::<Value>(r#"{"latte":{"num": 1}, "frappe": {"num": 2}, "macchiato": {"num": 3}, "mocha": {"num": 4}}"#).expect("valid json fixture");
    //     let result = render_and_parse_all(content, &reg, &ctx);
    //     assert_eq!(result, ("a: b\nc: d", expected, expected_root));
    // }
    //
}
