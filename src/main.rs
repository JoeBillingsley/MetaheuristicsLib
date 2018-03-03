extern crate rand;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;

use rand::{thread_rng, Rng};

use std::str;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use serde_json::Value;
use serde_json::Value::Number;

#[derive(Clone, Debug, PartialEq)]
struct Pub {
    name: &'static str,
    longitude: f64,
    latitude: f64,
 //   closing_time: i8,
}

static ALL_PUBS:[Pub; 3] = [
    Pub{name: "Imperial",longitude:2.0,latitude:3.0},
    Pub{name: "Chevalier",longitude:2.0,latitude:3.0},
    Pub{name: "Black Horse",longitude:2.0,latitude:3.0}
];

fn main() {
    // exhaustive_search();
    // blind_search();

    println!("{}", find_distance(&ALL_PUBS[0], &ALL_PUBS[1]));
}

fn blind_search() -> Vec<Pub> {
    
    let mut visited_pubs = Vec::new();

    loop {

        let remaining_pubs = get_available_pubs(&visited_pubs);

        if remaining_pubs.is_empty() {
            return visited_pubs;
        }

        let i = thread_rng().gen_range(0, remaining_pubs.len());

        visited_pubs.push(remaining_pubs[i].clone());
    }

}

fn exhaustive_search() -> Vec<Vec<Pub>> {

    return helper_search(Vec::new());

    fn helper_search(visited_pubs:Vec<Pub>) -> Vec<Vec<Pub>> {

        let mut aggr = Vec::new();
        let remaining_pubs = get_available_pubs(&visited_pubs); 

        if remaining_pubs.is_empty() {
            return vec!(visited_pubs)
        }

        for i in 0..remaining_pubs.len() {
            let mut new_pubs = visited_pubs.clone();
            new_pubs.push(remaining_pubs[i].clone());

            let ch_aggr = helper_search(new_pubs);

            for ch in ch_aggr {
                aggr.push(ch);
            }
        }

        return aggr;
    }
}

fn get_available_pubs(visited_pubs:&Vec<Pub>) -> Vec<Pub> {
    let mut available_pubs = Vec::new();

    for pub_ in ALL_PUBS.into_iter() { 

        if visited_pubs.contains(pub_) {
            continue;
        }

        available_pubs.push(pub_.clone());
    }

    available_pubs
}

fn find_distance(pub_one:&Pub, pub_two:&Pub) -> f64 {

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    
    let uri = "http://159.65.31.150/exeter/route/v1/walking/-3.5279128,50.7288359;-3.5350497,50.7226647".parse().unwrap();
    
    let work = client.get(uri).and_then(|res| {
        res.body().concat2().and_then(move |body| {
            let v: Value = serde_json::from_slice(&body).unwrap();
            
            Ok(v)
        })
    });
        
    let body = core.run(work).unwrap();

    body["routes"][0]["distance"].as_f64().unwrap()
}