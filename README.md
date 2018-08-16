# Metropogen
Copyright (c) 2018 Theodore Mason

Metropogen is a Rust-based tool to generate 24hrs of random events based on a
given JSON file defining context. Its intended use is to model sets of events
happening throughout an environ to more accurately represent a real, living
world for use in tabletop RPGs, worldbuilding exercises, or other creative
works.

### Installing and Building

Fork or clone the repository to any command-line environment capable of running
rust code. Build dependencies and run the project using:

    cargo build

### Usage

After building, execute metropogen with the default context with:

    cargo run

Specify a different context file with:

    cargo run -- -f myfile.json

Enable verbose mode to get a less compact output and view the longer description
of events:

    cargo run -- -v -f myfile.json

Where "myfile.json" is any properly-formatted context file. See the examples
provided or copy the template file to start building a custom context file.

### Context Files

Context files describe the environs to be simulated to the generator. The
default context file describes a small medieval town and its individual
buildings. Sample context files describing a nation of connected states and a
fleet of allied ships are also included. An empty context file is included to
serve as a reference or template. 

The structure of a properly-written context file is as such:

    {
        "name": "Name of the set",
        "landmarks": [

Describing the name of the set of items (a city, a battlefield, a country) as
well as the beginning of the list of items, followed by:

            {
                "name": "Name of the first item in the set",
                "events": [
                    {
                        "shortdesc": "The first event associated with this item",
                        "time_of_day": ["Morning", "EarlyDay", "MidDay", 
                            "LateDay", "Evening", "EarlyNight", "MidNight", "
                            LateNight"],
                        "rarity": 5,
                        "longdesc": "The longer description of the event,
                            typically 1-4 sentences."
                    }

Describing the name of the first item, followed by the first item associated
with the item. Notice that the only accepted values for "time_of_day" are
"Morning, EarlyDay, MidDay, LateDay, Evening, EarlyNight, MidNight, LateNight"
which are all case sensitive. time_of_day must be encapsulated in brackets even
if it only posseses one value. The rarity value must be an integer from 0-100
(inclusive). Events may continue after, defined the same way. Remember the comma
after finishing the event object before continuing to the next!

All together:

    {
        "name": "Name of the set",
        "landmarks": [
            {
                "name": "Name of the first item in the set",
                "events": [
                    {
                        "shortdesc": "The first event associated with this item",
                        "time_of_day": ["Morning", "EarlyDay", "MidDay", 
                            "LateDay", "Evening", "EarlyNight", "MidNight", "
                            LateNight"],
                        "rarity": 5,
                        "longdesc": "The longer description of the event,
                            typically 1-4 sentences."

                    }
                ]
            }
        ]
    }

### Future features
### Planned features:

    - Multi-layer (aristocratic, infrastructure, crime, government, etc)
        description of actions over a 24-hour period

    - Settings to tweak layer priorities (more "crime" actions, more "faith"
        actions, etc) or turn off layers (no aristocratic layer); change 
        size of city used (metropolis, city, large town, small town, village,
        township, hamlet, etc)

    - generation of "landmarks," locations that layers interact with that
        would be prominent in city

    - output to text file in useful format

### Stretch Goals:

    - read in user landmarks and use them at runtime

    - create setting "skins" like cyberpunk, low-magic fantasy, high-magic 
        fantasy, industrial era, etc that vastly change descriptions

    - create GUI version

# License
This work is licensed under the GPL v3.0 or later. See the LICENSE file for
further licensing terms.

