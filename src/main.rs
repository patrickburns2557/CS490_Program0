/**********************************
Patrick Burns
2022-09-05
CS 490 Programming Assignment 0

Purpose: Take in a radius input from 
the user and calculate the value of 
a sphere with that radius. 
**********************************/

const INCH_TO_CENTIMETER: f64 = 2.54;
const PI: f64 = 3.14159;
fn main() {
    println!("This program will calculate the volume of a sphere. ");
    let test: f64 = 1.0;
    println!("cm: {}", calculate_volume_cm(test));
    println!("in: {}", calculate_volume_in(test));
}


//This function calculates the volume of a sphere in square centimeters with the passed in radius
//Parameters: input_radius - radius of sphere in centimeters
//Returns: volume of sphere in square centimeters
fn calculate_volume_cm(input_radius: f64) -> f64 {
    (4.0 / 3.0) * PI * input_radius * input_radius * input_radius
}

fn calculate_volume_in(input_radius: f64) -> f64 {
    let inch_radius = input_radius / INCH_TO_CENTIMETER;
    (4.0 / 3.0) * PI * inch_radius * inch_radius * inch_radius
}