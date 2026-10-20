fn main() {
    let current_funds = 1000;
    let is_valid = verify_balance(current_funds);
    println!("Is data transfer valid ? {}", is_valid)
}
fn verify_balance(amount: u32) -> bool {
    amount > 5000
}