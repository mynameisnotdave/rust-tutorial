// Error handling. The second value of the generic is a string because the error returns
// a string message.
fn divide(x: f32, y: f32) -> Result<f32, &'static str> {
    if y == 0.0 {
        return Err("Division by zero");
    }
    Ok(x / y)
}
