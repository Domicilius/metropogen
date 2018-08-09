#![allow(dead_code)]
#![allow(unused_must_use)]
extern crate rand;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::fmt;
use std::cmp::Ordering;
//use std::env;
use std::fs::File;
//use std::io::prelude::*;
use rand::prelude::*;
//use serde_json::*;

// Make a standardized rarity structure so we can easily compare
// and handle events of different rarity levels
#[derive(Serialize, Deserialize, Debug, Eq)]
enum RarityValue {
    Ordinary,       // corresponds to   34  in 100
    Common,         // corresponds to   20  in 100
    Uncommon,       // corresponds to   10  in 100
    Rare,           // corresponds to   5   in 100
    Extraordinary,  // corresponds to   1   in 100
}

// Returning values of enums code heavily inspired by:
// https://stackoverflow.com/questions/36928569/enums-with-constant-values-in-rust
// by Huon and Shepmaster, among others.
impl RarityValue {
    fn value(&self) -> i32 {
        match *self {
            RarityValue::Ordinary => 1,
            RarityValue::Common => 2,
            RarityValue::Uncommon => 3,
            RarityValue::Rare => 4,
            RarityValue::Extraordinary => 5,
        }
    }
}

impl Ord for RarityValue {
    fn cmp(&self, other: &RarityValue) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for RarityValue {
    fn partial_cmp(&self, other: &RarityValue) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RarityValue {
    fn eq(&self, other: &RarityValue) -> bool {
        self.value() == other.value()
    }
}

// Make a standardized way to handle times of the day. We care more about
// what relative time-region of the day the event occurs in than the 
// absolute time of the event, to the point where the absolute itme is 
// basically an afterthought. Names of the relative regions are not quite
// accurate, but accurate enough to be usable.
#[derive(Serialize, Deserialize, Debug, Eq, Copy, Clone)]
enum Timezone {
    Morning,    // Morning      corresponds to 4AM  5AM  6AM
    EarlyDay,   // EarlyDay     corresponds to 7AM  8AM  9AM
    MidDay,     // MidDay       corresponds to 10AM 11AM 12PM
    LateDay,    // LateDay      corresponds to 1PM  2PM  3PM
    Evening,    // Evening      corresponds to 4PM  5PM  6PM
    EarlyNight, // EarlyNight   corresponds to 7PM  8PM  9PM
    MidNight,   // MidNight     corresponds to 10PM 11PM 12PM
    LateNight,  // LateNight    corresponds to 1AM  2AM  3AM
}

// Returning values of enums code heavily inspired by:
// https://stackoverflow.com/questions/36928569/enums-with-constant-values-in-rust
// by Huon and Shepmaster, among others.
impl Timezone {
    fn value(&self) -> i32 {
        match *self {
            Timezone::Morning => 1,
            Timezone::EarlyDay => 2,
            Timezone::MidDay => 3,
            Timezone::LateDay => 4,
            Timezone::Evening => 5,
            Timezone::EarlyNight => 6,
            Timezone::MidNight => 7,
            Timezone::LateNight => 8,
        }
    }

    fn string(&self) -> &str {
        match *self {
            Timezone::Morning => "Morning",
            Timezone::EarlyDay => "EarlyDay",
            Timezone::MidDay => "MidDay",
            Timezone::LateDay => "LateDay",
            Timezone::Evening => "Evening",
            Timezone::EarlyNight => "EarlyNight",
            Timezone::MidNight => "MidNight",
            Timezone::LateNight => "LateNight",
        }
    }

}

impl fmt::Display for Timezone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}
impl Ord for Timezone {
    fn cmp(&self, other: &Timezone) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Timezone {
    fn partial_cmp(&self, other: &Timezone) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Timezone {
    fn eq(&self, other: &Timezone) -> bool {
        self.value() == other.value()
    }
}

// Make a struct to handle the structure of a read-in event from JSON.
#[derive(Serialize, Deserialize, Debug, Eq)]
struct Event {
    shortdesc: String, //           "What's happening w/ this event"
    time_of_day: Vec<Timezone>, //  What time-region of the day it occurs
    longimpact: String, //          The long version of what's happening
    rarity: RarityValue //          How likely the event is to occur
    
    // TO IMPLEMENT
    //layer: String, //               The category of event occuring
    //exclusive: String, //           Whether or not multiple events of this
                                //  type can occur at the same time-region
}

impl Event {
    pub fn get_time(&self) -> &Timezone {
        &self.time_of_day[0]
    }

    pub fn get_shortdesc(&self) -> &String {
        &self.shortdesc
    }

    pub fn chance_happens(&self) -> &bool {
        match self.rarity {
            RarityValue::Ordinary => {
 				return weighted_choice([&true, &false].to_vec(), [34, 66].to_vec());
            }
            RarityValue::Common => {
 				return weighted_choice([&true, &false].to_vec(), [20, 80].to_vec());
            }
            RarityValue::Uncommon => {
 				return weighted_choice([&true, &false].to_vec(), [10, 90].to_vec());
            }
            RarityValue::Rare => {
 				return weighted_choice([&true, &false].to_vec(), [5, 95].to_vec());
            }
            RarityValue::Extraordinary => {
 				return weighted_choice([&true, &false].to_vec(), [1, 99].to_vec());
            }
        }
    }

    pub fn daygen(&self, giventime: Timezone) -> bool {
        ((self.chance_happens() == &true) && (self.time_of_day.contains(&giventime)))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event)-> Ordering {
        self.time_of_day[0].cmp(&other.time_of_day[0])
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.time_of_day[0] == other.time_of_day[0]
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Display function does not display strictly all the data
        // in the object, just the pertinent data necessary for displaying
        // it. Namely, it takes the timezone array that an event can occur
        // in and expands it out to figure out exactly when the event 
        // occurs in a 24-hr timeclock.
        
        let mut rng = thread_rng();
        let when;         
        

        // Figure out what hour of the day the event occurs based on its
        // time-region. Mostly important for display purposes (which is
        // why we're only figuring it out now!)
        match self.time_of_day[rng.gen_range(0, self.time_of_day.len())] {
            Timezone::Morning => {
                let choice = rng.gen_range(1, 4);
                match choice {
                    1 => {
                        when = "4AM";
                    },
                    2 => {
                        when = "5AM";
                    },
                    3 => {
                        when = "6AM";
                    }
                    _ => {
                        when = "Morning";
                    }
                }
                    
            },
            Timezone::EarlyDay => {
                let choice = rng.gen_range(1, 4);
                match choice {
                    1 => {
                        when = "7AM";
                    },
                    2 => {
                        when = "8AM";
                    },
                    3 => {
                        when = "9AM";
                    }
                    _ => {
                        when = "EarlyDay";
                    }
                }
            },
            Timezone::MidDay => {
                let choice = rng.gen_range(1, 4);
                match choice {
                    1 => {
                        when = "10AM";
                    },
                    2 => {
                        when = "11AM";
                    },
                    3 => {
                        when = "12PM";
                    }
                    _ => {
                        when = "MidDay";
                    }
                }
            },
            Timezone::LateDay => {
                let choice = rng.gen_range(1, 4);
                match choice {
                    1 => {
                        when = "1PM";
                    },
                    2 => {
                        when = "2PM";
                    },
                    3 => {
                        when = "3PM";
                    }
                    _ => {
                        when = "LateDay";
                    }
                }
            },
            Timezone::Evening => {
                let choice = rng.gen_range(1, 4);
                match choice {
                    1 => {
                        when = "4PM";
                    },
                    2 => {
                        when = "5PM";
                    },
                    3 => {
                        when = "6PM";
                    }
                    _ => {
                        when = "Evening";
                    }
                }
            },
            Timezone::EarlyNight => {
                let choice = rng.gen_range(1, 4);
                match choice {
                    1 => {
                        when = "7PM";
                    },
                    2 => {
                        when = "8PM";
                    },
                    3 => {
                        when = "9PM";
                    }
                    _ => {
                        when = "EarlyNight";
                    }
                }
            },
            Timezone::MidNight => {
                let choice = rng.gen_range(1, 4);
                match choice {
                    1 => {
                        when = "10PM";
                    },
                    2 => {
                        when = "11PM";
                    },
                    3 => {
                        when = "12AM";
                    }
                    _ => {
                        when = "MidNight";
                    }
                }
            },
            Timezone::LateNight => {
                let choice = rng.gen_range(1, 4);
                match choice {
                    1 => {
                        when = "1AM";
                    },
                    2 => {
                        when = "2AM";
                    },
                    3 => {
                        when = "3AM";
                    }
                    _ => {
                        when = "LateNight";
                    }
                }
            },
        }
        // Output the event in a formatted manner that makes it easy to
        // read.
	    write!(f, "[Event]    {}\n[When]    {}\n[Desc]    {}", self.shortdesc, when, self.longimpact)
    }
}

// Make an organized structure to hold the information about a read-in 
// landmark. Only really need name, location, and list of events, but
// more pieces can be added (owner or associated faction, primary layer,
// etc)
#[derive(Serialize, Deserialize, Debug)]
struct Landmark {
    name: String,
    events: Vec<Event>,

}

impl fmt::Display for Landmark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Output a Landmark in a formatted and easy to read style

        // Start with the header and name
        write!(f, "[Name]    {}\n============================\n\n", self.name);

        // Print each of the possible events of the landmark
        for i in &self.events {
            write!(f, "{}\n\n", i);
        }

        // return the fmt::Result
	    write!(f, "")
    }
}

impl Landmark {
    pub fn daygen(&self, giventime: Timezone) {
        for i in &self.events {
            let printer: bool = i.daygen(giventime);
            if printer {
                println!("  [{}] \t| [{}] \t| [{}]", giventime, self.name, i.shortdesc);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct City {
    name: String,
    landmarks: Vec<Landmark>,
}

impl City {
    fn daygen(&self) {
        println!("[{}]", self.name);
        for i in &self.landmarks {
            i.daygen(Timezone::Morning);
        }
        for i in &self.landmarks {
            i.daygen(Timezone::EarlyDay);
        }
        for i in &self.landmarks {
            i.daygen(Timezone::MidDay);
        }
        for i in &self.landmarks {
            i.daygen(Timezone::LateDay);
        }
        for i in &self.landmarks {
            i.daygen(Timezone::Evening);
        }
        for i in &self.landmarks {
            i.daygen(Timezone::EarlyNight);
        }
        for i in &self.landmarks {
            i.daygen(Timezone::MidNight);
        }
        for i in &self.landmarks {
            i.daygen(Timezone::LateNight);
        }
    }
}

/// Choose one of choices[] based on corresponding weight
fn weighted_choice<T>(choices: Vec<&T>, weights: Vec<i32>) -> &T {
    // Idea is to pick randomly from a list of choices but make the "random"
    // have more of a tendency to pick some choices from others.
    
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
    weighted_choices[rng.gen_range(0, 100)]
}

fn main() {
    // read in file in hopefully-good json format
    let file = File::open("config.json").unwrap();

    let json: City = serde_json::from_reader(file).unwrap();

    json.daygen();
    
}
