//include: variables, expressions, conditionals, loops, functions
fn main() {
    println!("running main");
    //prompt the user for a ceiling height and a lauch angle (and v0?)
    //for now, let's just give them values
    let height = 30.0;
    let theta: f32 = 3.14159265/6.0; //note that angles should be in radians... add in pi itself another time
    let v0 = 20.0;
    let mut distance = -1.0; // the mut tells the program that its value will change

    //calculate the farthest distance if it skims the ceiling
    distance = calculate_farthest_distance(height, theta, v0);
    //calculate flight time
    //stretch goal: plot the path if possible

    //for debugging purposes: print out all variables
    println!("height = {height}");
    println!("theta = {theta}");
    println!("cos(th) = ");
    println!("{}", theta.cos());
    println!("V0 = {v0}");
    println!("distance = {distance}")
}

fn calculate_farthest_distance(_y: f32, _th: f32, _v: f32) -> f32 {
    println!("running calculate_farthest_distance");
    //calculate the farthest distance here
    //return farthest distance
    return 5.4 //replace with calculated value later
}

fn calculate_flight_time() {
    println!("running calculate_flight_time");
    //calculate the flight time here
    //return flight time
}

fn calculate_path() {
    println!("running calculate_path");
    //calculate the path
    //return two lists: the x path and the y path
}

fn plot_path() {
    println!("running plot_path")
    //make a plot of the path using the two lists
}