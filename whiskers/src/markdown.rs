use itertools::Itertools as _;

use crate::templating::{self, Filter, Function};

pub trait TableDisplay {
    fn table_headings() -> Box<[String]>;
    fn table_row(&self) -> Box<[String]>;
}

#[must_use]
pub fn display_functions_as_table() -> String {
    let mut result = String::new();
    result.push_str("### Functions\n\n");
    result.push_str(display_as_table(&templating::all_functions()).as_str());
    result.push_str("\n### Filters\n\n");
    result.push_str(display_as_table(&templating::all_filters()).as_str());
    result
}

pub fn display_as_table<T: TableDisplay>(items: &[T]) -> String {
    let mut result = String::new();
    let rows = items.iter().map(T::table_row).collect::<Vec<_>>();

    // calculate a max width for each heading based on the longest row in the column
    let headings = T::table_headings();
    let headings = headings
        .iter()
        .enumerate()
        .map(|(i, heading)| {
            let max_width = rows
                .iter()
                .map(|row| row[i].len())
                .max()
                .unwrap_or(heading.len());
            (heading, max_width)
        })
        .collect::<Vec<_>>();

    // print the headings
    result.push_str(&format!(
        "| {} |\n",
        headings
            .iter()
            .map(|(heading, max_width)| format!("{heading:<max_width$}"))
            .join(" | ")
    ));

    // print the separator
    result.push_str(&format!(
        "| {} |\n",
        headings
            .iter()
            .map(|(_, max_width)| "-".repeat(*max_width))
            .join(" | ")
    ));

    // print the rows
    for row in rows {
        result.push_str(&format!(
            "| {} |\n",
            row.iter()
                .enumerate()
                .map(|(i, cell)| {
                    let max_width = headings[i].1;
                    format!("{cell:<max_width$}")
                })
                .join(" | ")
        ));
    }

    result
}

impl TableDisplay for Function {
    fn table_headings() -> Box<[String]> {
        Box::new([
            "Name".to_string(),
            "Description".to_string(),
            "Examples".to_string(),
        ])
    }

    fn table_row(&self) -> Box<[String]> {
        Box::new([
            format!("`{}`", self.name),
            self.description.clone(),
            if self.examples.is_empty() {
                "None".to_string()
            } else {
                self.examples.first().map_or_else(String::new, |example| {
                    format!(
                        "`{name}({input})` ⇒ `{output}`",
                        name = self.name,
                        input = example
                            .inputs
                            .iter()
                            .map(|(k, v)| format!("{k}={v}"))
                            .join(", "),
                        output = example.output
                    )
                })
            },
        ])
    }
}

impl TableDisplay for Filter {
    fn table_headings() -> Box<[String]> {
        Box::new([
            "Name".to_string(),
            "Description".to_string(),
            "Examples".to_string(),
        ])
    }

    fn table_row(&self) -> Box<[String]> {
        Box::new([
            format!("`{}`", self.name),
            self.description.clone(),
            if self.examples.is_empty() {
                "None".to_string()
            } else {
                self.examples.first().map_or_else(String::new, |example| {
                    if example.inputs.is_empty() {
                        format!(
                            "`{value} \\| {name}` ⇒ `{output}`",
                            value = example.value,
                            name = self.name,
                            output = example.output
                        )
                    } else {
                        format!(
                            "`{value} \\| {name}({input})` ⇒ `{output}`",
                            value = example.value,
                            name = self.name,
                            input = example
                                .inputs
                                .iter()
                                .map(|(k, v)| format!("{k}={v}"))
                                .join(", "),
                            output = example.output
                        )
                    }
                })
            },
        ])
    }
}
