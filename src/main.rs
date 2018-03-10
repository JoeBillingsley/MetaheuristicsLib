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

#[derive(Clone, Debug, PartialEq)]
struct Pub {
    name: &'static str,
    longitude: f64,
    latitude: f64,
    closing_time: f64,
}

static ALL_PUBS:[Pub; 3] = [
    Pub{name: "Imperial",longitude:-3.5279378,latitude:50.7310101,opening:10,closing:3},
    Pub{name: "Chevalier",longitude:-3.5279128,latitude:50.7288359,opening:10,closing:3},
    Pub{name: "Black Horse",longitude:-3.5315411,latitude:50.7252184,opening:10,closing:3}
];

fn main() {
    // exhaustive_search();
    // blind_search();

    println!("{:?}", greedy_search(ALL_PUBS[0].clone()));
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

fn greedy_search(starting_pub:Pub) -> Vec<Pub> {

    let mut visited_pubs = vec![starting_pub];

    loop {
        let remaining_pubs = get_available_pubs(&visited_pubs);

        if remaining_pubs.is_empty() {
            return visited_pubs;
        }

        let mut nearest_pub = 0;
        let mut nearest_dist = std::f64::INFINITY;
        
        for i in 0..remaining_pubs.len() {
            let dist = find_distance(&visited_pubs[visited_pubs.len()-1], &remaining_pubs[i]);

            if dist < nearest_dist {
                nearest_dist = dist;
                nearest_pub = i;
            }
        }

        visited_pubs.push(remaining_pubs[nearest_pub].clone());
    }
}

fn get_available_pubs(visited_pubs:&Vec<Pub>, time:f64) -> Vec<Pub> {
    let mut available_pubs = Vec::new();

    for pub_ in ALL_PUBS.into_iter() { 

        if visited_pubs.contains(pub_) {
            continue;
        }

        available_pubs.push(pub_.clone());
    }

    available_pubs
}

fn find_distance(pub_one:&Pub, pub_two:&Pub) -> (f64, f64) {

    // TODO: Add request caching

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    
    let req = format!(
        "http://159.65.31.150/exeter/route/v1/walking/{},{};{},{}", 
        pub_one.longitude, pub_one.latitude, pub_two.longitude, pub_two.latitude);

    let uri = req.parse().unwrap();
    
    let work = client.get(uri).and_then(|res| {
        res.body().concat2().and_then(move |body| {
            let v: Value = serde_json::from_slice(&body).unwrap();
            
            Ok(v)
        })
    });
        
    let body = core.run(work).unwrap();

    let dist = body["routes"][0]["distance"].as_f64().unwrap();
    let time = body["routes"][0]["duration"].as_f64().unwrap();

    return (dist, time);
}

fn is_open(pub_:&Pub, curr_time:f64) -> bool {
    curr_time > pub_.opening && curr_time < pub_.closing
}

fn add_seconds(time:f64, seconds:f64) -> f64 {

}