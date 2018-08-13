#![allow(dead_code)]
#![allow(unused_must_use)]
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate textwrap;

#[macro_use]
extern crate serde_derive;

use std::fmt;
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use rand::prelude::*;
use clap::{Arg, App};
use textwrap::fill;

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
    
    // Have Timezone enums return an associated integer so that we can sort lists of
    // Timezones. Sorting allows associated events to have any order of Timezones
    // listed in their description in the associated JSON file.
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

    // Have Timezone enums return an associated string value so that we can display
    // them in a beautiful, human-readable format.
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

	fn clockvalue(&self) -> &str {
        
        let when;         
		let mut rng = thread_rng();
		
		match *self {
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
		when
	}
}

// Overwrite the default display function for Timezones so that we can print them
// Structure of function derived from official rust documentation on
// implementing custom display functions for enums
impl fmt::Display for Timezone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

// Implement comparison functions for Timezone based on their associated values
// so that events can be sorted by their relative timing during the day. We want
// to sort our events so that we can output events chronologically, which is the only
// sane way for the user to see the events happening over a day.
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
    shortdesc: String,          //  The name of the event, for practical purposes         
    time_of_day: Vec<Timezone>, //  The relative time of the day for the event to occur.
                                //  Support a vec of Timezones because events might have
                                //  a range of times they occur.
    rarity: i32,                //  An integer representing the event's chance to occur out
                                //  of 100. Does not support values above 100.
	longdesc: String			//  The longer description of the event that describes in 
								//  relative detail what occurs during this event. Only output
                                //  if verbose flag is flipped. 
    
}

impl Event {
    
    // Function to return an Event's Timezone so that we can sort Events based on when they
    // can occur during the day. Returns the first Timezone in the Event's time_of_day value
    // so Events with large time ranges may seem sorted out of place.
    pub fn get_time(&self) -> &Timezone {
        &self.time_of_day[0]
    }

    // Function to return an Event's shortdesc, which we're using as its name
    pub fn get_shortdesc(&self) -> &String {
        &self.shortdesc
    }

    // Function to determine if an Event happens based on its rarity value. Returns
    // a true or false bool value.
    pub fn chance_happens(&self) -> &bool {
 	    weighted_choice([&true, &false].to_vec(), [self.rarity, (100-self.rarity)].to_vec())
    }

    // Function to handle the determination of whether an Event happens given a Timezone.
    // Used in conjunction with loops to iterate through a whole day and determine if any
    // event occurs in each Timezone.
    pub fn daygen(&self, giventime: Timezone) -> bool {
        ((self.chance_happens() == &true) && (self.time_of_day.contains(&giventime)))
    }
}

// Implement ordering and comparison functions for Event based on Timezone values
// so that we can sort vectors of Events based on their relative chronological ordering
// throughout the order.
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
        
	// Structure of function derived from official rust documentation on
	// implementing custom display functions for enums
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Output the event in a formatted manner that makes it easy to
        // read for the user.
	    write!(f, "[Event]    {}\n[When]    {:?}", self.shortdesc, self.time_of_day)
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
        
    // Function to print a Landmark enum in an easy to read format
	// Structure of function derived from official rust documentation on
	// implementing custom display functions for enums
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

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
    
    // Function to determine the events occuring in regards to a Landmark given a Timezone
    // value. Used in conjunction with loops and City values to figure out which Events occur
    // on any given day.
    pub fn daygen(&self, giventime: Timezone, verbose: bool) {

        // Check each Event
        for i in &self.events {

            // Figure out if the Event occurs in the given Timezone
            let chancehappens: bool = i.daygen(giventime);
            
            // If the event is happening, print out the Event in a formmatted manner
            if chancehappens {
				if verbose {
					println!("Where:\t[{}]\nWhat:\t[{}]\nWhen:\t[{}]\n---\n{}\n", self.name, i.shortdesc, giventime.clockvalue(), fill(&i.longdesc, 80));
				} else {
					println!("[{}] \t| [{}] \t| [{}]", giventime.clockvalue(), self.name, i.shortdesc);
				}
            }
        }
    }
}

// Struct to hold a vec of Landmarks and a common name to hold the collection.
// While the struct is called City, there isn't anything stopping the collection
// being a region, some large temple, or any other collection of Landmarks. It doesn't
// even have to be a collection, the name could even be just "My Landmarks"
#[derive(Serialize, Deserialize, Debug)]
struct City {
    name: String,
    landmarks: Vec<Landmark>,
}

impl City {

    // Function to determine all of the events occurring in a collection of Landmarks
    // in a day. Run through each of the 8 Timezones we're using to represent a 24-hr
    // day and check all of the Event objects contained in the city to determine
    // if they're occurring. The daygen functions in Event and Landmark will handle
    // all of the actual printing, so we only need to output the name of the City
    // we're checking. We're taking the verbose command line argument and passing
    // it down the line so we know how to output later on.
    fn daygen(&self, verbose: bool) {
        println!("[{}]", self.name);
        for i in &self.landmarks {
            i.daygen(Timezone::Morning, verbose);
        }
        for i in &self.landmarks {
            i.daygen(Timezone::EarlyDay, verbose);
        }
        for i in &self.landmarks {
            i.daygen(Timezone::MidDay, verbose);
        }
        for i in &self.landmarks {
            i.daygen(Timezone::LateDay, verbose);
        }
        for i in &self.landmarks {
            i.daygen(Timezone::Evening, verbose);
        }
        for i in &self.landmarks {
            i.daygen(Timezone::EarlyNight, verbose);
        }
        for i in &self.landmarks {
            i.daygen(Timezone::MidNight, verbose);
        }
        for i in &self.landmarks {
            i.daygen(Timezone::LateNight, verbose);
        }
    }
}

/// Choose one of choices[] based on corresponding weight
// Idea is to pick randomly from a list of choices but make the "random"
// have more of a tendency to pick some choices from others.
fn weighted_choice<T>(choices: Vec<&T>, weights: Vec<i32>) -> &T {
    
    // If there isn't a weight for every choice, crash. It's possible
    // for us to infer weights in some cases if they're missing, but we
    // have no way to handle missing choices and other corner cases, so
    // just make sure we're working with the right inputs from the start.
    assert_eq!(choices.len(), weights.len());

    // Create a new list of choices, adding a given element of choice
    // once for the number of times given in weight. Essentially make a list
    // of 100 elements, with elements corresponding to members of choices
    // and appearing more than once if they have a higher weight. This lets
    // us just continue to use rand's built in functions to get the result
    // we require.
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

	// Set default file to be used if user does not pass in a config file. This file
	// is stored in src/config.json, but user could be running program from anywhere,
	// so we need to create a relative path based off the source directory of metropogen	
	let cargomanifestdir = env!("CARGO_MANIFEST_DIR");
	env::set_current_dir(cargomanifestdir);
    let defaultconfigfile = "src/config.json";
    
	// command line argument parsing structure derived from example in clap's crate documentation
    // found on their github here: https://github.com/clap-rs/clap
    let matches = App::new("Metropogen")
                          .version("1.0")
                          .author("Theodore Mason <mason.theodorej@gmail.com>")
                          .about("Generate 24hr of random events based on user inputfiles")
                          .arg(Arg::with_name("INPUT")
							   .short("f")
							   .long("file")
							   .value_name("FILE")
                               .help("Identify input file if not using default")
                               .required(false)
							   .takes_value(true))
                          .arg(Arg::with_name("verbose")
                               .short("v")
							   .long("verbose")
                               .help("Set verbose output to true"))
                          .get_matches();

    // Use user's configuration file if they gave us one, otherwise use default
	let configfile = matches.value_of("config").unwrap_or(&defaultconfigfile);

    // Figure out if we're printing in verbose mode or not 
	let mut verbose: bool = false;
	if matches.is_present("verbose") {
		verbose = true;
	}

    // Structure to read JSON object from file derived from serde documentation
    // found on their github: https://github.com/serde-rs/serde
    // read in file in hopefully-good json format
    let file = File::open(configfile).unwrap();

    let json: City = serde_json::from_reader(file).unwrap();

    // process the read-in information
    json.daygen(verbose);

}
