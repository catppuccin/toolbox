use std::collections::HashMap;

#[derive(Debug)]
pub struct Document {
    pub frontmatter: HashMap<String, tera::Value>,
    pub body: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid YAML frontmatter (L{line}:{column}) : {message}")]
    InvalidYaml {
        line: usize,
        column: usize,
        message: String,
    },
}

pub fn parse(input: &str) -> Result<Document, Error> {
    let Some((frontmatter, body)) = split(input) else {
        // no frontmatter to parse
        return Ok(Document {
            frontmatter: HashMap::new(),
            body: input.to_string(),
        });
    };

    Ok(Document {
        frontmatter: serde_yaml::from_str(frontmatter).map_err(|e| Error::InvalidYaml {
            line: e.location().map(|l| l.line()).unwrap_or_default(),
            column: e.location().map(|l| l.column()).unwrap_or_default(),
            message: e.to_string(),
        })?,
        body: body.to_string(),
    })
}

fn split(template: &str) -> Option<(&str, &str)> {
    // we consider a template to possibly have frontmatter iff:
    // * line 0 is "---"
    // * there is another "---" on another line
    let sep = "---\n";
    if !template.starts_with(sep) {
        return None;
    }

    template[sep.len()..].split_once(sep)
}
