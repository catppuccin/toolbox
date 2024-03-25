use indexmap::IndexMap;

use crate::{filters, functions};

/// Allows creation of a [`FilterExample`] with the following syntax:
///
/// `function_example!(mix(base=base, blend=red, amount=0.5) => "#804040")`
macro_rules! function_example {
    ($name:ident($($key:ident = $value:tt),*) => $output:expr) => {
        $crate::templating::FunctionExample {
            inputs: {
                let mut map = indexmap::IndexMap::new();
                $(map.insert(stringify!($key).to_string(), stringify!($value).to_string());)*
                map
            },
            output: $output.to_string(),
        }
    };
}

/// Allows creation of a [`FilterExample`] with the following syntax:
///
/// `filter_example!(red | add(hue=30)) => "#ff6666")`
macro_rules! filter_example {
    ($value:ident | $name:ident => $output:expr) => {
        $crate::templating::FilterExample {
            value: stringify!($value).to_string(),
            inputs: indexmap::IndexMap::new(),
            output: $output.to_string(),
        }
    };
    ($value:ident | $name:ident($($key:ident = $arg_value:tt),*) => $output:expr) => {
        $crate::templating::FilterExample {
            value: stringify!($value).to_string(),
            inputs: {
                let mut map = indexmap::IndexMap::new();
                $(map.insert(stringify!($key).to_string(), stringify!($arg_value).to_string());)*
                map
            },
            output: $output.to_string(),
        }
    };
}

pub fn make_engine() -> tera::Tera {
    let mut tera = tera::Tera::default();
    tera.register_filter("add", filters::add);
    tera.register_filter("sub", filters::sub);
    tera.register_filter("mod", filters::modify);
    tera.register_filter("urlencode_lzma", filters::urlencode_lzma);
    tera.register_function("mix", functions::mix);
    tera.register_function("if", functions::if_fn);
    tera.register_function("object", functions::object);
    tera
}

#[must_use]
pub fn all_functions() -> Vec<Function> {
    vec![
        Function {
            name: "mix".to_string(),
            description: "Mix two colors together".to_string(),
            examples: vec![
                function_example!(mix(base=base, blend=red, amount=0.5) => "#804040"),
                function_example!(mix(base=base, blend=red, amount=0.5) => "#804040"),
            ],
        },
        Function {
            name: "if".to_string(),
            description: "Return one value if a condition is true, and another if it's false"
                .to_string(),
            examples: vec![
                function_example!(if(cond=true, t=1, f=0) => "1"),
                function_example!(if(cond=false, t=1, f=0) => "0"),
            ],
        },
        Function {
            name: "object".to_string(),
            description: "Create an object from the input".to_string(),
            examples: vec![
                function_example!(object(a=1, b=2) => "{a: 1, b: 2}"),
                function_example!(object(a=1, b=2) => "{a: 1, b: 2}"),
            ],
        },
    ]
}

#[must_use]
pub fn all_filters() -> Vec<Filter> {
    vec![
        Filter {
            name: "add".to_string(),
            description: "Add a value to a color".to_string(),
            examples: vec![
                filter_example!(red | add(hue=30) => "#ff6666"),
                filter_example!(red | add(saturation=0.5) => "#ff6666"),
            ],
        },
        Filter {
            name: "sub".to_string(),
            description: "Subtract a value from a color".to_string(),
            examples: vec![
                filter_example!(red | sub(hue=30) => "#ff6666"),
                filter_example!(red | sub(saturation=0.5) => "#ff6666"),
            ],
        },
        Filter {
            name: "mod".to_string(),
            description: "Modify a color".to_string(),
            examples: vec![
                filter_example!(red | mod(lightness=0.5) => "#ff6666"),
                filter_example!(red | mod(opacity=0.5) => "#ff6666"),
            ],
        },
        Filter {
            name: "urlencode_lzma".to_string(),
            description: "Serialize an object into a URL-safe string with LZMA compression"
                .to_string(),
            examples: vec![
                filter_example!(red | urlencode_lzma => "#ff6666"),
                filter_example!(red | urlencode_lzma => "#ff6666"),
            ],
        },
    ]
}

#[derive(serde::Serialize)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub examples: Vec<FunctionExample>,
}

#[derive(serde::Serialize)]
pub struct Filter {
    pub name: String,
    pub description: String,
    pub examples: Vec<FilterExample>,
}

#[derive(serde::Serialize)]
pub struct FunctionExample {
    pub inputs: IndexMap<String, String>,
    pub output: String,
}

#[derive(serde::Serialize)]
pub struct FilterExample {
    pub value: String,
    pub inputs: IndexMap<String, String>,
    pub output: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn function_example_with_single_arg() {
        let example = function_example!(mix(base=base) => "#804040");
        assert_eq!(example.inputs["base"], "base");
        assert_eq!(example.output, "#804040");
    }

    #[test]
    fn function_example_with_multiple_args() {
        let example = function_example!(mix(base=base, blend=red, amount=0.5) => "#804040");
        assert_eq!(example.inputs["base"], "base");
        assert_eq!(example.inputs["blend"], "red");
        assert_eq!(example.inputs["amount"], "0.5");
        assert_eq!(example.output, "#804040");
    }

    #[test]
    fn filter_example_with_no_args() {
        let example = filter_example!(red | add => "#ff6666");
        assert_eq!(example.value, "red");
        assert_eq!(example.inputs.len(), 0);
        assert_eq!(example.output, "#ff6666");
    }

    #[test]
    fn filter_example_with_single_arg() {
        let example = filter_example!(red | add(hue=30) => "#ff6666");
        assert_eq!(example.value, "red");
        assert_eq!(example.inputs["hue"], "30");
        assert_eq!(example.output, "#ff6666");
    }

    #[test]
    fn filter_example_with_multiple_args() {
        let example = filter_example!(red | add(hue=30, saturation=0.5) => "#ff6666");
        assert_eq!(example.value, "red");
        assert_eq!(example.inputs["hue"], "30");
        assert_eq!(example.inputs["saturation"], "0.5");
        assert_eq!(example.output, "#ff6666");
    }
}
