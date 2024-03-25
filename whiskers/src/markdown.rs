use itertools::Itertools as _;

use crate::templating;

#[derive(Clone, Copy)]
pub enum Format {
    List,
    Table,
}

#[must_use]
pub fn format_filters_and_functions(format: Format) -> String {
    match format {
        Format::List => list_format(),
        Format::Table => table_format(),
    }
}

fn list_format() -> String {
    let mut result = String::new();
    result.push_str("## Functions\n\n");
    for function in templating::all_functions() {
        result.push_str(&format!(
            "### `{name}`\n\n{description}\n\n",
            name = function.name,
            description = function.description
        ));
        if !function.examples.is_empty() {
            result.push_str("#### Examples\n\n");
            for example in &function.examples {
                result.push_str(&format!(
                    "- `{name}({input})` => `{output}`\n",
                    name = function.name,
                    input = example
                        .inputs
                        .iter()
                        .map(|(k, v)| format!("{k}={v}"))
                        .join(", "),
                    output = example.output
                ));
            }
            result.push('\n');
        }
    }

    result.push_str("## Filters\n\n");
    for filter in templating::all_filters() {
        result.push_str(&format!(
            "### `{name}`\n\n{description}\n\n",
            name = filter.name,
            description = filter.description
        ));
        if !filter.examples.is_empty() {
            result.push_str("#### Examples\n\n");
            for example in &filter.examples {
                result.push_str(&format!(
                    "- `{value} | {name}({input})` => `{output}`\n",
                    value = example.value,
                    name = filter.name,
                    input = example
                        .inputs
                        .iter()
                        .map(|(k, v)| format!("{k}={v}"))
                        .join(", "),
                    output = example.output
                ));
            }
            result.push('\n');
        }
    }

    result
}

fn table_format() -> String {
    let mut result = String::new();
    result.push_str("## Functions\n\n");
    result.push_str("| Name | Description | Examples |\n");
    result.push_str("|------|-------------|----------|\n");
    for function in templating::all_functions() {
        result.push_str(&format!(
            "| `{name}` | {description} | {examples} |\n",
            name = function.name,
            description = function.description,
            examples = if function.examples.is_empty() {
                "None".to_string()
            } else {
                function
                    .examples
                    .first()
                    .map_or_else(String::new, |example| {
                        format!(
                            "`{name}({input})` => `{output}`",
                            name = function.name,
                            input = example
                                .inputs
                                .iter()
                                .map(|(k, v)| format!("{k}={v}"))
                                .join(", "),
                            output = example.output
                        )
                    })
            }
        ));
    }

    result.push_str("## Filters\n\n");
    result.push_str("| Name | Description | Examples |\n");
    result.push_str("|------|-------------|----------|\n");
    for filter in templating::all_filters() {
        result.push_str(&format!(
            "| `{name}` | {description} | {examples} |\n",
            name = filter.name,
            description = filter.description,
            examples = if filter.examples.is_empty() {
                "None".to_string()
            } else {
                filter.examples.first().map_or_else(String::new, |example| {
                    format!(
                        "`{value} \\| {name}({input})` => `{output}`",
                        value = example.value,
                        name = filter.name,
                        input = example
                            .inputs
                            .iter()
                            .map(|(k, v)| format!("{k}={v}"))
                            .join(", "),
                        output = example.output
                    )
                })
            }
        ));
    }

    result
}
