/**********************************
Patrick Burns
2022-09-05
CS 490 Programming Assignment 0

Purpose: Take in a radius input from 
the user and calculate the value of 
a sphere with that radius. 
**********************************/

use std::io;

const INCH_TO_CENTIMETER: f64 = 2.54;

fn main() {
    println!("This program will calculate the volume of a sphere. ");
    println!("You can type \"quit\" at any time to close the program.");
    
    //infinite loop
    //can be exited if the user inputs "quit"
    loop {
        //create variable to store user input
        let mut input = String::new();
        println!("Enter the radius of your sphere (in centimeters):");

        //read in input from user, with an error message if it fails
        io::stdin().read_line(&mut input).expect("Failed to read input.");

        //check if the user inputs "quit" and break out of the loop if so (end program)
        if input.contains("quit") {
            break;
        }

        //convert user input into a f64 variable.
        //If there's any error in doing so, continues to next iteration of loop
        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //Print out the results using the calculation functions
        println!("\n Input radius: {input} cm");
        println!(" Volume: {:.3} cm^3", calculate_volume_cm(input)); //uses "{:.3}" to limit the output to 3 decimal places
        println!(" Volume: {:.3} in^3", calculate_volume_in(input));
        println!("");

    }


}


//This function calculates the volume of a sphere in square centimeters with the passed in radius
//Parameters: input_radius - radius of sphere in centimeters
//Returns: volume of sphere in square centimeters
fn calculate_volume_cm(input_radius: f64) -> f64 {
    (4.0 / 3.0) * std::f64::consts::PI * input_radius * input_radius * input_radius //uses RUST's built in PI constant
}


//This function calculates the volume of a sphere in cubic inches with the passed in radius
//Parameters: input_radius - radius of sphere in centimeters
//Returns: volume of sphere in cubic inches
fn calculate_volume_in(input_radius: f64) -> f64 {
    let inch_radius = input_radius / INCH_TO_CENTIMETER; //convert cm to inches
    (4.0 / 3.0) * std::f64::consts::PI * inch_radius * inch_radius * inch_radius //uses RUST's built in PI constant
}