use std::collections::HashMap;

#[derive(Debug)]
struct MacroMetadata {
    name: String,
    params: Vec<String>,
    param_w_value: HashMap<String, Option<String>>,
    body: Vec<String>,  
} 

#[derive(Debug)]
pub struct MacroProcessor {
    macros: Vec<MacroMetadata>
}

impl MacroProcessor {
    pub fn new() -> Self {
        Self { macros: Vec::new() }
    }

    pub fn macro_process(&mut self, source_lines: &[String]) -> Vec<String> {
        let mut expanded_lines = Vec::new();
        let mut iter = source_lines.iter().peekable();

        while let Some(line) = iter.next() {
            let trimmed_line = line.trim();
            let mut tokens = trimmed_line.split_whitespace();
            if let Some(token) = tokens.next() {
                if token == "MACRO" {
                    let macro_metadata = self.parse_macro(&mut iter);
                    self.macros.push(macro_metadata);
                    continue;
                }

                if let Some(macro_metadata) = self.find_macro(token) {
                    let expanded = self.expand_macro(macro_metadata, tokens);
                    expanded_lines.extend(expanded);
                    continue;
                }
            }
            expanded_lines.push(trimmed_line.to_string());
        }
        println!("{:?}", self.macros);
        for line in &expanded_lines {
            println!("{}", line);
        }
        expanded_lines
    }

    fn parse_macro<'a, I>(&self, iter: &mut std::iter::Peekable<I>) -> MacroMetadata
    where
        I: Iterator<Item = &'a String>,
    {
        let header_line = iter.next().unwrap();
        let mut tokens = header_line.split_whitespace();

        let name = tokens.next().unwrap().to_string();
        let mut params = Vec::new();
        let mut param_w_value = std::collections::HashMap::new();
        for param in tokens {
            if let Some((key, value)) = param.split_once('=') {
                param_w_value.insert(key.to_string().clone(), Some(value.to_string()));
                params.push(key.to_string());
            } else {
                param_w_value.insert(param.to_string(), None);
                params.push(param.to_string());
            }
            
        }

        let mut body = Vec::new();
        while let Some(line) = iter.next() {
            let trimmed_line = line.trim();
            if trimmed_line == "MACROEND" {
                break;
            }
            body.push(trimmed_line.to_string());
        }

        MacroMetadata {
            name,
            params,
            param_w_value,
            body,
        }
    }

    fn find_macro(&self, name: &str) -> Option<&MacroMetadata> {
        self.macros.iter().find(|macro_metadata| macro_metadata.name == name)
    }

    fn expand_macro<'a, I>(&self, macro_metadata: &MacroMetadata, mut tokens: I) -> Vec<String>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut resolved_params = macro_metadata.param_w_value.clone();

        for (i, param_value) in tokens.enumerate() {
            if let Some(formal_param) = macro_metadata.params.get(i) {
                resolved_params.insert(formal_param.clone(), Some(param_value.to_string()));
            }
        }

        println!("{:?}", resolved_params);

        macro_metadata
            .body
            .iter()
            .map(|line| {
                let mut expanded_line = line.clone();
                for (formal_param, value) in &resolved_params {
                    if let Some(actual_value) = value {
                        expanded_line = expanded_line.replace(formal_param, actual_value);
                    }
                }
                expanded_line
            })
            .collect()
        }
}