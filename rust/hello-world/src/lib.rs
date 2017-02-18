
pub fn hello(name: Option<&str>) -> String {
    match name {
        Some(name) => format!("Hello, {}!", name).to_string(),
        None => "Hello, World!".to_string(),
    }
}
