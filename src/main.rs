#![recursion_limit="256"]
#![feature(box_patterns)]

extern crate chrono;
extern crate egg_mode;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;

mod common;

use chrono::prelude::*;
use common::tokio_core::reactor;
use egg_mode::tweet::Tweet;
use dotenv::dotenv;
use std::env;
use std::{thread, time};

fn main() {
    let mut core = reactor::Core::new().unwrap();

    let config = common::Config::load(&mut core);
    let handle = core.handle();

    let user_id = config.user_id;
    let mut idk =
        core.run(
            egg_mode::user::relation(
                "bitemyapp",
                "symbolic_undead",
                &config.token,
                &handle)
        ).unwrap().response;
    println!("{:?}", idk);
    // let mut status = core.run(egg_mode::tweet::show(tweet_id, &config.token, &handle)).unwrap();
    // common::print_tweet(&status);
    // let tweet_id = 766678057788829697;

    // println!("");
    // println!("Load up an individual tweet:");
    // let status = core.run(egg_mode::tweet::show(tweet_id, &config.token, &handle)).unwrap();
    // common::print_tweet(&status);

    // println!("");
    // println!("Loading retweets of an individual tweet:");
    // for rt in &core.run(egg_mode::tweet::retweets_of(tweet_id, 5, &config.token, &handle)).unwrap() {
    //     if let Some(ref user) = rt.user {
    //         println!("{} (@{})", user.name, user.screen_name);
    //     }
    // }

    // println!("");
    // println!("Loading the user's home timeline:");
    // let mut home = egg_mode::tweet::home_timeline(&config.token, &handle).with_page_size(5);
    // for status in &core.run(home.start()).unwrap() {
    //     common::print_tweet(&status);
    //     println!("");
    // }

    // println!("");
    // println!("Loading the user's mentions timeline:");
    // let mut home = egg_mode::tweet::mentions_timeline(&config.token, &handle).with_page_size(5);
    // for status in &core.run(home.start()).unwrap() {
    //     common::print_tweet(&status);
    //     println!("");
    // }

    // println!("");
    // println!("Loading the user's timeline:");
    // let mut home = egg_mode::tweet::user_timeline(config.user_id, true, true,
    //                                               &config.token, &handle).with_page_size(5);
    // for status in &core.run(home.start()).unwrap() {
    //     common::print_tweet(&status);
    //     println!("");
    // }
}
