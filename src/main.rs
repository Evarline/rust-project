fn calculate_rectangle (width:i32, height:i32) -> i32 {
    let area = width * height;
    return area;
}
fn main() {
    let width = 8;
    let height = 5;
    let area = calculate_rectangle(width, height);
    println!("The area of the rectangle is: {}", area);
}
