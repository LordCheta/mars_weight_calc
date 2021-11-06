use std::io;

fn main() {

    println!("Please enter your weight in kg: ");
    let mut user_weight_on_earth = String::new();

    io::stdin().read_line(&mut user_weight_on_earth).unwrap();

    let user_weight_on_earth = user_weight_on_earth.trim().parse::<f32>().unwrap();

    let user_weight_on_mars = calculate_weight_on_mars(user_weight_on_earth);

    println!("Your weight on mars is {}kg", user_weight_on_mars);
}

fn calculate_weight_on_mars(weight_on_earth: f32) -> f32 {
    let mars_weight: f32 = (weight_on_earth / 9.81) * 3.711;

    mars_weight
}
