pub fn hello(name: Option<&str>) -> String {
    let name = match name {
        Some(name) => name,
        None => "World"
    };

    format!("Hello, {}!", name)
}