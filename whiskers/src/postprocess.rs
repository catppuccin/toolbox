use base64::Engine;

struct UnquoteReplacer;
impl regex::Replacer for UnquoteReplacer {
    fn replace_append(&mut self, caps: &regex::Captures, dst: &mut String) {
        let Ok(bytes) = &base64::engine::general_purpose::STANDARD_NO_PAD.decode(&caps["b64"])
        else {
            eprintln!(
                "warning: failed to decode whiskers unquote section. this is probably a bug."
            );
            return;
        };
        let json = String::from_utf8_lossy(bytes);
        dst.push_str(&json);
    }
}

#[must_use]
#[allow(clippy::missing_panics_doc)] // a panic in here means we wrote the regex wrong
pub fn postprocess(input: &str) -> String {
    let pattern = regex::Regex::new(r#""\{WHISKERS:UNQUOTE:(?<b64>.*)}""#).expect("regex is valid");
    let result = pattern.replace_all(input, UnquoteReplacer).to_string();
    result
}
