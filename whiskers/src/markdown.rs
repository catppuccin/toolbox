use itertools::Itertools as _;

use crate::templating;

#[must_use]
pub fn display_functions_as_list() -> String {
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

pub fn display_functions_as_table() -> String {
    let mut result = String::new();
    result.push_str("### Functions\n\n");
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

    result.push_str("\n### Filters\n\n");
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
                    if example.inputs.is_empty() {
                        format!(
                            "`{value} \\| {name}` => `{output}`",
                            value = example.value,
                            name = filter.name,
                            output = example.output
                        )
                    } else {
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
                    }
                })
            }
        ));
    }

    result
}
