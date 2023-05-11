//include: variables, expressions, conditionals, loops, functions
fn main() {
    println!("running main");
    //g is the acceleration due to gravity
    let g = 9.81; //units: m/s^2
    //prompt the user for a ceiling height and a lauch angle (and v0?)
    //for now, let's just give them values
    let height = 30.0;
    let theta: f32 = 3.14159265/6.0; //note that angles should be in radians... add in pi itself another time
    let v0 = 20.0;
    let mut distance = -1.0; // the mut tells the program that its value will change

    //calculate the farthest distance if there's no ceiling
    distance = calculate_distance_no_ceiling(g, theta, v0); //wip
    //calculate the farthest distance if it skims the ceiling
    //distance = calculate_distance_with_ceiling(g, height, theta, v0); //wip

    //calculate flight time
    //stretch goal: plot the path if possible

    //for debugging purposes: print out all variables
    println!("height = {height}");
    println!("theta = {theta}");
    println!("cos(th) = ");
    println!("{}", theta.cos());
    println!("V0 = {v0}");
    println!("distance = {distance}");
    
    // testing sqrt()
    println!("square root of 9: ");
    let testing: f32 = 9.0;
    println!("{}", 9.0_f32.sqrt());
    println!("{}", testing.sqrt());
}

fn calculate_distance_no_ceiling(g: f32, th: f32, v: f32) -> f32 {
    println!("running calculate_distance_no_ceiling");
    //calculate the farthest distance here
    // the equation is explained in the README *****add it to the README********
    let angle = 2.0 * th;
    let xf = v * v * angle.sin() / g;
    
    println!("angle = {angle}");
    println!("distance = {xf}");
    return xf 
}
fn calculate_distance_with_ceiling(g: f32, y: f32, th: f32, v: f32) -> f32 {
    println!("running calculate_distance_with_ceiling");
    //calculate the farthest distance here
    // the a, b, and c are from an equation explained in the README *****add it to the README********
    let a = - 0.25 * g;
    let b = v * th.cos() * th.sin();
    let c =  - 2.0 * y * v * v * th.cos() * th.cos();
    // something about sqrt: the thing you're taking a square root of needs explicitly be a float
    let inside_sqrt: f32 = b*b - (4.0 * a * c);
    //let positive = (-b + inside_sqrt.sqrt() ) / (2 * a);
    //let negative = (-b - inside_sqrt.sqrt() ) / (2 * a);
    // positive = (-b + sqrt(b*b - 4.0 * a * c) ) / (2 * a)
    // negative = (-b - sqrt(b*b - 4.0 * a * c) ) / (2 * a) //shouldn't be a thing here

    println!("a = {a} and b = {b} and c = {c}");
    println!("inside the squre root = {inside_sqrt}");
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