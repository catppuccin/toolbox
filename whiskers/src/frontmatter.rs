use handlebars::Handlebars;
use serde_json::Value;

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
    fn render_frontmatter() {
        let content = "---\na: {{var}}\nc: d\n---\na: b\nc: d\n";
        let expected =
            serde_json::from_str::<Value>(r#"{"a":"b","c":"d"}"#).expect("valid json fixture");
        let reg = Handlebars::new();
        let ctx = serde_json::from_str::<Value>(r#"{"var":"b"}"#).expect("valid json fixture");
        let result = render_and_parse(content, &reg, &ctx);
        assert_eq!(result, ("a: b\nc: d", Some(expected)));
    }
}
