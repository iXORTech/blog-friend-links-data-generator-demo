use serde_json::Value;
use regex::Regex;
use std::collections::HashSet;

/// Convert a Vec of serde_json::Value to JavaScript object string.
pub fn json_to_js_object(data: &Vec<Value>) -> String {
    json_to_js_format(&Value::Array(data.clone()), 0)
}

/// Recursively convert serde_json::Value to JavaScript format string.
fn json_to_js_format(obj: &Value, indent_level: usize) -> String {
    let indent = "  ".repeat(indent_level);
    let next_indent = "  ".repeat(indent_level + 1);
    
    match obj {
        Value::Object(map) => {
            if map.is_empty() {
                return "{}".to_string();
            }
            
            let items: Vec<String> = map.iter().map(|(key, value)| {
                let js_key = if is_valid_js_identifier(key) {
                    key.clone()
                } else {
                    format!("\"{}\"", key)
                };
                
                let js_value = json_to_js_format(value, indent_level + 1);
                format!("{}{}: {}", next_indent, js_key, js_value)
            }).collect();
            
            format!("{{\n{}\n{}}}", items.join(",\n"), indent)
        }
        
        Value::Array(arr) => {
            if arr.is_empty() {
                return "[]".to_string();
            }
            
            let items: Vec<String> = arr.iter().map(|item| {
                let js_item = json_to_js_format(item, indent_level + 1);
                format!("{}{}", next_indent, js_item)
            }).collect();
            
            format!("[\n{}\n{}]", items.join(",\n"), indent)
        }
        
        Value::String(s) => {
            let escaped = s
                .replace("\\", "\\\\")
                .replace("\"", "\\\"")
                .replace("\n", "\\n")
                .replace("\r", "\\r")
                .replace("\t", "\\t");
            format!("\"{}\"", escaped)
        }
        
        Value::Bool(b) => {
            if *b { "true".to_string() } else { "false".to_string() }
        }
        
        Value::Null => "null".to_string(),
        
        Value::Number(n) => n.to_string()
    }
}

/// Check if a string is a valid JavaScript identifier.
fn is_valid_js_identifier(name: &str) -> bool {
    // JavaScript identifier regex pattern
    let re = Regex::new(r"^[a-zA-Z_$][a-zA-Z0-9_$]*$").unwrap();
    
    // JavaScript reserved words
    let reserved_words: HashSet<&str> = [
        "class", "const", "let", "var", "function", "return", "if", "else", 
        "for", "while", "do", "switch", "case", "default", "break", "continue", 
        "try", "catch", "finally", "throw", "new", "this", "super", "extends", 
        "import", "export", "from", "as", "async", "await", "yield", "static", 
        "public", "private", "protected"
    ].iter().cloned().collect();
    
    re.is_match(name) && !reserved_words.contains(name)
}
