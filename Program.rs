//include: variables, expressions, conditionals, loops, functions
fn main() {
    println!("running main");
    //prompt the user for a ceiling height and a lauch angle (and v0?)
    //for now, let's just give them values
    let height = 30;
    let theta: f32 = 3.14159265/6.0; //note that angles should be in radians... add in pi itself another time

    //calculate the farthest distance if it skims the ceiling
    //calculate flight time
    //stretch goal: plot the path if possible

    //for debugging purposes: print out all variables
    println!("height = {height}");
    println!("theta = {theta}");
    println!("cos(th) = ");
    println!("{}", theta.cos());
}

fn calculate_farthest_distance() {
    println!("running calculate_farthest_distance")
    //calculate the farthest distance here
    //return farthest distance
}

fn calculate_flight_time() {
    println!("running calculate_flight_time")
    //calculate the flight time here
    //return flight time
}

fn calculate_path() {
    println!("running calculate_path")
    //calculate the path
    //return two lists: the x path and the y path
}

fn plot_path() {
    println!("running plot_path")
    //make a plot of the path using the two lists
}