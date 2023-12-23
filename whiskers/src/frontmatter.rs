use handlebars::Handlebars;
use json_patch::merge;
use serde_json::Value;

use crate::{Map, COLOR_NAMES, FLAVOR_NAMES};

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
fn merge_overrides(cli_overrides: Option<Value>, frontmatter: Value, flavor: &str) -> Value {
    let mut merged_mut = frontmatter.clone();

    // applying cli overrides
    let mut merged = cli_overrides.map_or(frontmatter.clone(), |cli| {
        let mut merged = frontmatter;
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
        .expect("merged can be converted to a mutable map");

    // Don't need the "overrides" block anymore since we've hoisted everything up
    merged_mut.remove("overrides");

    // Propagate overridden palette colors to inside ["colors] handlebars iterator
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

    serde_json::to_value(merged_mut).expect("overridden frontmatter can be serialized")
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
            FLAVOR_NAMES
                .map(|v| (v.into(), Value::Null))
                .into_iter()
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

    let parsed: Value = match serde_yaml::from_str(frontmatter) {
        Ok(frontmatter) => frontmatter,
        Err(e) => {
            eprintln!("warning: Failed to parse YAML frontmatter ({e}). Proceeding without it.");
            return (content, Value::Null);
        }
    };

    let overridden = merge_overrides(overrides, parsed, flavor);

    let rendered = match reg.render_template(&overridden.to_string(), ctx) {
        Ok(frontmatter) => frontmatter,
        Err(e) => {
            eprintln!(
                "warning: Failed to render frontmatter templates ({e}). Proceeding without it"
            );
            return (content, Value::Null);
        }
    };

    match serde_yaml::from_str(&rendered) {
        Ok(frontmatter) => (content, frontmatter),
        Err(e) => {
            eprintln!("warning: Failed to parse YAML frontmatter ({e}). Proceeding without it.");
            (content, Value::Null)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

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
        let result = render_and_parse(content, overrides, "mocha", &reg, &ctx);
        assert_eq!(result, ("a: b\nc: d", expected));
    }

    #[test]
    fn parse_frontmatter_with_cli_overrides() {
        let content = "---\na: b\nc: d\n---\na: b\nc: d\n";
        let expected = serde_json::from_str::<Value>(r#"{"a":"override","c":"d"}"#)
            .expect("valid json fixture");
        let reg = Handlebars::new();
        let ctx = Value::Object(Map::new());
        let overrides = Some(json!({"a": "override"}));
        let result = render_and_parse(content, overrides, "mocha", &reg, &ctx);
        assert_eq!(result, ("a: b\nc: d", expected));
    }

    #[test]
    fn parse_frontmatter_with_override_block() {
        let content = "---\na: b\nc: d\noverrides:\n  mocha:\n    a: 'override'\n---\na: b\nc: d\n";
        let expected = serde_json::from_str::<Value>(r#"{"a":"override","c":"d"}"#)
            .expect("valid json fixture");
        let reg = Handlebars::new();
        let ctx = Value::Object(Map::new());
        let overrides = Some(Value::Object(Map::new()));
        let result = render_and_parse(content, overrides, "mocha", &reg, &ctx);
        assert_eq!(result, ("a: b\nc: d", expected));
    }

    mod merge_overrides {
        use crate::frontmatter::merge_overrides;
        use serde_json::{json, Value};

        macro_rules! yaml {
            ($yaml:expr) => {{
                serde_yaml::from_str::<Value>($yaml).expect("yaml can be parsed")
            }};
        }

        #[test]
        fn frontmatter_with_no_overrides() {
            let frontmatter = yaml!(
                r#"
                    accent: "{{ mauve }}"
                    primary: true
            "#
            );
            let actual = merge_overrides(None, frontmatter.clone(), "mocha");
            assert_eq!(actual, frontmatter);
        }

        #[test]
        fn frontmatter_with_single_flavor_override_and_is_current_flavor() {
            let frontmatter = yaml!(
                r#"
                    accent: "{{ mauve }}"
                    primary: true
                    overrides:
                        mocha:
                            accent: "{{ blue }}"
            "#
            );
            let expected = yaml!(
                r#"
                    accent: "{{ blue }}"
                    primary: true
            "#
            );
            let actual = merge_overrides(None, frontmatter, "mocha");
            assert_eq!(actual, expected);
        }

        #[test]
        fn frontmatter_with_single_flavor_override_and_is_not_current_flavor() {
            let frontmatter = yaml!(
                r#"
                    accent: "{{ mauve }}"
                    primary: true
                    overrides:
                        mocha:
                            accent: "{{ blue }}"
            "#
            );
            let expected = yaml!(
                r#"
                    accent: "{{ mauve }}"
                    primary: true
            "#
            );
            let actual = merge_overrides(None, frontmatter, "latte");
            assert_eq!(actual, expected);
        }

        #[test]
        fn frontmatter_with_all_flavors_override() {
            let frontmatter = yaml!(
                r#"
                    accent: "{{ mauve }}"
                    primary: true
                    overrides:
                        all:
                            accent: "{{ blue }}"
            "#
            );
            let expected = yaml!(
                r#"
                    accent: "{{ blue }}"
                    primary: true
            "#
            );
            let actual = merge_overrides(None, frontmatter, "mocha");
            assert_eq!(actual, expected);
        }

        #[test]
        fn frontmatter_with_palette_colours() {
            let frontmatter = yaml!(
                r#"
                    accent: "{{ mauve }}"
                    primary: true
                    overrides:
                        mocha:
                            accent: "{{ blue }}"
                            base: "020202"
                            mantle: "010101"
                            crust: "000000"
            "#
            );
            let expected = yaml!(
                r#"
                    accent: "{{ blue }}"
                    base: "020202"
                    mantle: "010101"
                    crust: "000000"
                    primary: true
                    colors:
                        base: "020202"
                        mantle: "010101"
                        crust: "000000"
            "#
            );
            let actual = merge_overrides(None, frontmatter, "mocha");
            assert_eq!(actual, expected);
        }

        #[test]
        fn cli() {
            let frontmatter = yaml!(r#"accent: "{{ mauve }}""#);
            let overrides = Some(json!({
                "accent": "{{ pink }}"
            }));
            let actual = merge_overrides(overrides.clone(), frontmatter, "mocha");
            assert_eq!(Some(actual), overrides);
        }

        #[test]
        fn cli_overriding_frontmatter() {
            let frontmatter = yaml!(
                r#"
                    accent: "{{ mauve }}"
                    user: "sgoudham"
                    overrides:
                        mocha:
                            accent: "{{ blue }}"

            "#
            );
            let overrides = Some(json!({
                "accent": "{{ pink }}"
            }));
            let expected = yaml!(
                r#"
                    accent: "{{ pink }}"
                    user: "sgoudham"
            "#
            );
            let actual = merge_overrides(overrides, frontmatter, "mocha");
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn render_frontmatter() {
        let content = "---\na: '{{var}}'\nc: d\n---\n{{a}}\nc: d\n";
        let expected =
            serde_json::from_str::<Value>(r#"{"a":"b","c":"d"}"#).expect("valid json fixture");
        let reg = Handlebars::new();
        let ctx = serde_json::from_str::<Value>(r#"{"var":"b"}"#).expect("valid json fixture");
        let overrides = Some(Value::Object(Map::new()));
        let result = render_and_parse(content, overrides, "mocha", &reg, &ctx);
        assert_eq!(result, ("{{a}}\nc: d", expected));
    }

    #[test]
    fn render_frontmatter_when_flavor_all() {
        let content = "---\na: '{{num}}'\nc: d\n---\n{{a}}\nc: d\n";
        let expected = json!({
            "latte": {"a": "1","c": "d"},
            "frappe": {"a": "2","c": "d"},
            "macchiato": {"a": "3","c": "d"},
            "mocha": {"a": "4","c": "d"}
        })
        .as_object()
        .expect("expected is valid json")
        .clone();
        let reg = Handlebars::new();
        let ctx = serde_json::from_str::<Value>(r#"{"latte":{"num": 1}, "frappe": {"num": 2}, "macchiato": {"num": 3}, "mocha": {"num": 4}}"#).expect("valid json fixture");
        let overrides = Some(Value::Object(Map::new()));
        let result = render_and_parse_all(content, &overrides, &reg, &ctx);
        assert_eq!(result, ("{{a}}\nc: d", expected));
    }
}
