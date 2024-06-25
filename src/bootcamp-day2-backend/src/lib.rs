#[ic_cdk::query]
fn greet(name: String, num: i8) -> String {
    format!("Hello, {name} {num}!")
}
