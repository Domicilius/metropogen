extern crate rand;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::fmt;
//use std::env;
use std::fs::File;
//use std::io::prelude::*;
use rand::prelude::*;
//use serde_json::*;

#[derive(Serialize, Deserialize, Debug)]
enum RarityValue {
    Ordinary,       // corresponds to   34  in 100
    Common,         // corresponds to   20  in 100
    Uncommon,       // corresponds to   10  in 100
    Rare,           // corresponds to   5   in 100
    Extraordinary,  // corresponds to   1   in 100
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    shortdesc: String,
    layer: String,
    exclusive: String,
    time_of_day: Vec<Timezone>,
    longimpact: String,
    rarity: RarityValue
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
	    write!(f, "[Event]\t{}\n[Layer]\t{}\n[When]\t{}\n[Desc]\t{}", self.shortdesc, self.layer, when, self.longimpact)
    }

}


#[derive(Serialize, Deserialize, Debug)]
struct Landmark {
    name: String,
    events: Vec<Event>,

}

impl fmt::Display for Landmark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
   
        write!(f, "[Name]\t{}\n============================\n\n", self.name);
        for i in &self.events {
            write!(f, "{}\n\n", i);
        }
    

	    write!(f, "")
    }
}
    

/// Choose one of choices[] based on corresponding weight
fn weighted_choice(choices: Vec<&str>, weights: Vec<i32>) -> &str {
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

    let json: Landmark = serde_json::from_reader(file).unwrap();

    println!("{}", json);

}
