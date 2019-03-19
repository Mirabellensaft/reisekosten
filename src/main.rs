#[macro_use]
extern crate html5ever;

use std::default::Default;
use std::iter::repeat;
use std::string::String;
use std::env;
use std::fs::{self, File};

use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::tendril::TendrilSink;

// This is not proper HTML serialization, of course.

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let city = &args[1];
    let country = &args[2];
    let filename = &args[3];
    println!("Searching for {} in {}", escape_default(city), escape_default(country));
    println!("in file {}", filename);

    let mut contents = File::open(filename)
        .expect("Something went wrong");

    //let buffered = BufReader::new(contents);


    //let stdin = io::stdin();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut contents)
        .unwrap();

    let mut this_query = Query {
        city: city.to_string(),
        country: country.to_string(),
        counter: 0,
    };

    walk(dom.document, &mut this_query);

    if !dom.errors.is_empty() {
        println!("\nParse errors:");
        for err in dom.errors.into_iter() {
            println!("    {}", err);
        }
    }
}


fn walk(handle: Handle, query: &mut Query ) {

    let node = handle;

    if query.counter == 4 {
        // make output
    }

    if query.counter > 0_i32 && query.counter < 4_i32 {

        match node.data {
            NodeData::Text { ref contents } => {

                if escape_default(&contents.borrow()) != escape_default("\n") {
                    println!("{}  ", escape_default(&contents.borrow()));
                    //append to string
                    query.counter += 1_i32;
                }
            },

            _ => (),

        }
    }

    match node.data {
        NodeData::Text { ref contents } => {

            if escape_default(&contents.borrow()) == query.city {
                println!("{} ", escape_default(&contents.borrow()));
                query.counter = 1_i32;

            } else if escape_default(&contents.borrow()) == query.country {
                println!("{} ", escape_default(&contents.borrow()));
                query.counter = 1_i32;
            }
        },

    _ => (),
    }

    for child in node.children.borrow().iter() {
        walk(child.clone(), query);
    }
}

struct Query {
    city: String,
    country: String,
    counter: i32,
}

struct Output {
    city: String,
    country: String,
    whole_day: i32,
    travel_day: i32,
    over_night: i32,
}


// FIXME: Copy of str::escape_default from std, which is currently unstable
pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}
