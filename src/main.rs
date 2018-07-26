extern crate rand;
extern crate serde;
extern crate serde_yaml;
extern crate serde_derive;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use rand::prelude::*;

/// Choose one of choices[] based on corresponding weight
// Idea is to pick randomly from a list of choices but make the "random"
// have more of a tendency to pick some choices from others.
fn weighted_choice(choices: Vec<&str>, weights: Vec<i32>) -> &str {
    // If there isn't a weight for every choice, crash
    assert_eq!(choices.len(), weights.len());

    // Create a new list of choices, adding a given element of choice
    // once for the number of times given in weight. Essentially make a list
    // of 100 elements, with elements corresponding to members of choices
    // and appearing more than once if they have a higher weight.
    let mut weighted_choices = Vec::new();
    let mut index = 0;
    for i in weights.iter() {
        for _j in 0..*i {
            weighted_choices.push(choices[index]);
        }
        index += 1;
    }
    
    // use rand to pick one from our new list
    let mut rng = thread_rng();
//    println!("{}", weighted_choices[rng.gen_range(0, 100)]);
    weighted_choices[rng.gen_range(0, 100)]
}

fn main() {
    // read in file in hopefully-good json format
//    let mut file = File::open("config.yaml").unwrap();
//    let mut data = String::new();
//    file.read_to_string(&mut data);

//    let yaml: String = serde_yaml::from_str(&data).unwrap();

//    let mut contents = String::new();
//    file.read_to_string(&mut contents);

//    println!("{:?}", yaml);

}
