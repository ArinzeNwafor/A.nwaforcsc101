use std::io;

fn Area_of_Trap() {
    println!("\nInput the parameters needed:");
    // input parameters
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\nInput the height of the trapezium in cm: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let height:f64 = input1.trim().parse().expect("Not a valid integer");

    println!("Input the value of the first base of the trapezium in cm: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let base1:f64 = input2.trim().parse().expect("Not a valid integer");

    println!("Input the value of the second base of the trapezium in cm: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let base2:f64 = input3.trim().parse().expect("Not a valid integer");

    let area:f64 = (height * (base1 + base2)) / 2.0;

    println!("\nThe area of the trapezium is {}cm²", area);
}


fn Area_of_Rhom() {
    println!("\nInput the parameters needed:");
    // input parameters
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nInput the first diagonal of the rhombus in cm: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let diag1:f64 = input1.trim().parse().expect("Not a valid integer");

    println!("Input the second diagonal of the rhombus in cm: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let diag2:f64 = input2.trim().parse().expect("Not a valid integer");


    let area:f64 = 0.5 * diag1 * diag2;

    println!("\nThe area of the Rhombus is {}cm²", area);

}


fn Area_of_Parall() {
    println!("\nInput the parameters needed:");
    // input parameters
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nInput the base of the parallelogram in cm: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let base:f64 = input1.trim().parse().expect("Not a valid integer");

    println!("Input the altitude of the parallelogram in cm: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let alt:f64 = input2.trim().parse().expect("Not a valid integer");


    let area:f64 = base * alt;

    println!("\nThe area of the Parallelogram is {}cm²", area);
}

fn Area_of_Cube() {
    println!("\nInput the parameters needed:");
    // input parameters
    let mut input1 = String::new();

    println!("\nInput the length of one side of the cube in cm: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let length:f64 = input1.trim().parse().expect("Not a valid integer");

    let area:f64 = 6.0 * (length).powf(2.0);

    println!("\nThe area of the Cube is {}cm²", area);
}

fn Volume_of_Cylinder() {
    println!("\nInput the parameters needed:");
    // input parameters
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nInput the radius of the cylinder in cm: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let radius:f64 = input1.trim().parse().expect("Not a valid integer");

    println!("Input the height of the cylinder in cm: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let height:f64 = input2.trim().parse().expect("Not a valid integer");


    let volume:f64 = (22.0/7.0) * radius.powf(2.0) * height;

    println!("\nThe area of the Rhombus is {}cm³", volume);
}

fn main() {
    //introduction
    println!("Below is a list of equations. You are kindly prompted to pick an equation 
and input the necessary parameters that are needed to solve the equation.");
    println!("\n1. Area of Trapezium formula = (height*(base1 + base2))/2
2. Area of a Rhombus formula = 1⁄2 * diagonal1 * diagonal2
3. Area of Parallelogram formula = base * altitude
4. Area of Cube formula = 6 * (length of the side)²
5. Volume of Cylinder formula = π * radius² * height");

    // ask user for desired equation
    let mut input1 = String::new();
    println!("\nWhich equation would you like to use?(Choose between 1 to 5 by inputing 1,2,3,4 or 5): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let selection = input1.trim();

    if selection == "1" {
        Area_of_Trap();
    }
    else if selection == "2" {
        Area_of_Rhom();
    }
    else if selection == "3" {
        Area_of_Parall();
    }
    else if selection == "4" {
        Area_of_Cube();
    }
    else if selection == "5" {
        Volume_of_Cylinder();
    }

}