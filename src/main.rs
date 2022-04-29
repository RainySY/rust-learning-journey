fn greet_world() {
    let chinese = "世界你好";
    let regions = [chinese];
    for region in regions.iter() {
        println!("{}",&region);
    }
}
fn main() {
    greet_world();
}
