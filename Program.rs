//include: conditionals
use std::io; //lets us use input/output capabilities

fn main() {
    //g is the acceleration due to gravity
    let g = 9.81; //units: m/s^2
    //prompt the user for a ceiling height, launch angle, and launch velocity
    println!("Please enter a height in meters: ");
    let height = convert_to_float(); //not needed unless I incorporate ceiling height
    println!("Please enter an angle in degrees: ");
    let degrees = convert_to_float();
    let theta: f32 = degrees * 3.14159265 / 180.0; //converts the degrees into radians
    println!("Please enter a launching velocity in meters/second: ");
    let v0 = convert_to_float();
    //for now, let's just give them values
    //let height = 30.0;
    //let theta: f32 = 3.14159265/6.0; //note: angles should be in radians
    //let v0 = 20.0;
    let mut distance = -1.0; // the mut tells the program that its value will change
    let mut flight_time = -1.0; //I use the -1 value as a way to check whether the variable is reassigned later

    //calculate the farthest distance if there's no ceiling
    distance = calculate_distance_no_ceiling(g, theta, v0);

    //calculate flight time
    flight_time = calculate_flight_time(distance, theta, v0);
    //plot the path if possible
    let x_points = calculate_x_path(theta, v0, flight_time);
    let y_points = calculate_y_path(theta, v0, flight_time, g);

    println!("The projectile traveled {} meters across the room in {} seconds. ", distance, flight_time);
    //for debugging purposes: print out variables
    //println!("xpoints = {:?}", x_points);
    //println!("ypoints = {:?}", y_points);
}
fn convert_to_float() -> f32 {
    let mut input_text = String::new();
    //reads the inputed text as a String
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    //converts the String to a float
    let trimmed = input_text.trim();
    let value: f32 = match trimmed.parse::<f32>() {
        Ok(i) => i,
        Err(..) => -1.0,
    };
    return value
}
fn calculate_distance_no_ceiling(g: f32, th: f32, v: f32) -> f32 {
    // the equation is explained in the README file
    let angle = 2.0 * th;
    let xf = v * v * angle.sin() / g;
    
    //println!("angle = {angle}");
    println!("distance = {xf}");
    return xf 
}
/********** stretch goal: take the ceiling into account
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
}*/

fn calculate_flight_time(xf: f32, th: f32, v: f32) -> f32 {
    //equation from kinematic equation xf = xi + v0 * cos(theta) * t
    let tf = xf / (v * th.cos() );
    return tf
}

fn calculate_x_path(th: f32, v: f32, tf: f32) -> [f32;21] { //20 time data points
    //the values -1 are used specifically because that value wouldn't physically make sense
    let mut x_points: [f32;21] = [-1.0; 21];
    let mut t = -1.0;

    //notation for range is [starting point(inclusive)]..[ending point(exclusive)]
    for i in 0..21 {  
        t = (i as f32 * tf) / 20.0; // the "as f32" converts the integer to a float so it can be multiplied to another float        
        x_points[i] = v * th.cos() * t //x = v0 * cos(theta) * t
    }
    return x_points
}

fn calculate_y_path(th: f32, v: f32, tf: f32, g: f32) -> [f32;21] {
    //the values -1 are used specifically because that value wouldn't physically make sense
    let mut y_points: [f32;21] = [-1.0; 21];
    let mut t = -1.0;

    //notation for range is [starting point(inclusive)]..[ending point(exclusive)]
    for i in 0..21 {  
        t = (i as f32 * tf) / 20.0;        
        y_points[i] = v * th.sin() * t  - 0.5 * g * t * t //y = v0 * sin(theta) * t - 1/2 g t^2
    }
    return y_points
}

fn plot_path() {
    println!("running plot_path")
    //make a plot of the path using the two lists
}