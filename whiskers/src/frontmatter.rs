use handlebars::Handlebars;
use serde_json::{Map, Value};

pub type FlavorContexts = Vec<Option<Value>>;
pub type RootContext = Option<Map<String, Value>>;

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

#[must_use]
#[allow(clippy::missing_panics_doc)] // panic here implies an internal issue
pub fn render_and_parse_all<'a>(
    template: &'a str,
    reg: &Handlebars,
    ctx: &Value,
) -> (&'a str, FlavorContexts, RootContext) {
    let Some((frontmatter, content)) = split(template) else {
        return (template, vec![None], None);
    };

    let parsed_frontmatter = match serde_yaml::from_str::<Map<_, _>>(frontmatter) {
        Ok(frontmatter) => frontmatter,
        Err(e) => {
            eprintln!("warning: Failed to parse frontmatter as YAML ({e}). Proceeding without it");
            return (content, vec![None], None);
        }
    };

    let pattern = regex::Regex::new(r"\{\{.*?\}\}").expect("regex is valid");
    let root_context_frontmatter = parsed_frontmatter
        .into_iter()
        .filter(|(_, v)| !pattern.is_match(v.to_string().as_str()))
        .collect::<Map<String, Value>>();

    let frontmatter = ctx
        .as_object()
        .expect("context is an object")
        .values()
        .map(|v| render_and_parse(template, reg, v).1)
        .collect::<Vec<_>>();

    // Remove variables defined in the root context from each flavor
    let frontmatter = frontmatter
        .iter()
        .map(|value| {
            value.as_ref().map_or_else(
                || None,
                |frontmatter| {
                    let frontmatter_without_root = frontmatter
                        .as_object()
                        .expect("frontmatter is an object")
                        .clone()
                        .into_iter()
                        .filter(|(k, _)| !root_context_frontmatter.contains_key(k.as_str()))
                        .collect::<Map<_, _>>();
                    Some(
                        serde_json::to_value(frontmatter_without_root)
                            .expect("frontmatter is serializable"),
                    )
                },
            )
        })
        .collect::<Vec<_>>();

    (content, frontmatter, Some(root_context_frontmatter))
}

#[must_use]
pub fn render_and_parse<'a>(
    template: &'a str,
    reg: &Handlebars,
    ctx: &Value,
) -> (&'a str, Option<Value>) {
    let Some((frontmatter, content)) = split(template) else {
        return (template, None);
    };

    let frontmatter = match reg.render_template(frontmatter, ctx) {
        Ok(frontmatter) => frontmatter,
        Err(e) => {
            eprintln!(
                "warning: Failed to render frontmatter templates ({e}). Proceeding without it"
            );
            return (content, None);
        }
    };

    match serde_yaml::from_str(&frontmatter) {
        Ok(frontmatter) => (content, Some(frontmatter)),
        Err(e) => {
            eprintln!("warning: Failed to parse YAML frontmatter ({e}). Proceeding without it.");
            (content, None)
        }
    }
}

#[cfg(test)]
mod tests {
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
        let ctx = Value::Object(serde_json::Map::new());
        let result = render_and_parse(content, &reg, &ctx);
        assert_eq!(result, ("a: b\nc: d", Some(expected)));
    }

    #[test]
    fn parse_root_context_frontmatter_when_flavor_all() {
        let content = "---\na: b\nc: d\n---\na: b\nc: d\n";
        let expected =
            serde_json::from_str::<Map<_, _>>(r#"{"a":"b","c":"d"}"#).expect("valid json fixture");
        let reg = Handlebars::new();
        let ctx = Value::Object(serde_json::Map::new());
        let result = render_and_parse_all(content, &reg, &ctx);
        assert_eq!(result, ("a: b\nc: d", vec![], Some(expected)));
    }

    #[test]
    fn render_frontmatter() {
        let content = "---\na: {{var}}\nc: d\n---\na: b\nc: d\n";
        let expected =
            serde_json::from_str::<Value>(r#"{"a":"b","c":"d"}"#).expect("valid json fixture");
        let reg = Handlebars::new();
        let ctx = serde_json::from_str::<Value>(r#"{"var":"b"}"#).expect("valid json fixture");
        let result = render_and_parse(content, &reg, &ctx);
        assert_eq!(result, ("a: b\nc: d", Some(expected)));
    }

    #[test]
    fn render_frontmatter_when_flavor_all() {
        let content = "---\na: '{{num}}'\nc: d\n---\na: b\nc: d\n";
        let expected: Vec<Option<Value>> = vec![
            Some(serde_json::json!({"a": "1"})),
            Some(serde_json::json!({"a": "2"})),
            Some(serde_json::json!({"a": "3"})),
            Some(serde_json::json!({"a": "4"})),
        ];
        let expected_root: Option<Map<String, Value>> =
            serde_json::json!({"c": "d"}).as_object().cloned();
        let reg = Handlebars::new();
        let ctx = serde_json::from_str::<Value>(r#"{"latte":{"num": 1}, "frappe": {"num": 2}, "macchiato": {"num": 3}, "mocha": {"num": 4}}"#).expect("valid json fixture");
        let result = render_and_parse_all(content, &reg, &ctx);
        assert_eq!(result, ("a: b\nc: d", expected, expected_root));
    }
}
