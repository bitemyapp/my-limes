#![recursion_limit="256"]
#![feature(box_patterns)]

extern crate chrono;
extern crate egg_mode;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;

mod common;

use common::futures::Stream;
use chrono::prelude::*;
use chrono::{NaiveDateTime, NaiveDate};
use common::tokio_core::reactor;
use egg_mode::cursor;
use egg_mode::error::Error;
use egg_mode::user;
use egg_mode::tweet::Tweet;
use dotenv::dotenv;
use std::collections::HashSet;
use std::env;
use std::{thread, time};

fn main() {
    let mut core = reactor::Core::new().unwrap();

    let config = common::Config::load(&mut core);
    let handle = core.handle();

    let user_id = config.user_id;
    let idk =
        core.run(
            egg_mode::user::relation(
                &user_id,
                "symbolic_undead",
                &config.token,
                &handle)
        ).unwrap().response;
    println!("{:?}", idk);
//    for friend in core.run(user::friends_of(&user_id, &config.token, &handle)).unwrap() {
//       println!("{:?}", friend);
//    }
//    core.run(user::friends_of(&user_id, &config.token, &handle))
//        .map(|r| r.response)
//        .for_each(|friend| { println!("{:?}", friend); Ok(())}).unwrap();

    println!("");
    let mut friends = HashSet::new();
    core.run(user::friends_ids(config.user_id, &config.token, &handle)
        .map(|r| r.response)
        .for_each(|id| { friends.insert(id); Ok(()) })).unwrap();

    let mut followers = HashSet::new();
    let followers_cursor =
        user::followers_ids(config.user_id, &config.token, &handle).with_page_size(5000);
    let mut done = false;
    while !done {
        let follow_resp = core.run(followers_cursor.call());
        match follow_resp {
            Ok(resp) => for id in resp.response.ids {
                followers.insert(id);
            },
            Err(err) => match err {
                Error::RateLimit(epoch) => {
                    println!("We got a rate limit response! We're sleeping until {:?} and then retrying", epoch);
                    println!("Here are the friends we collected so far: {:?}", friends);
                    println!("Here are the followers we collected so far: {:?}", followers);
//                    let now = time::Instant::now();
//                    let epoch_time = NaiveDateTime::from_timestamp(epoch.into(), 0);
//                    // let sleepy_time = now.duration_until(epoch_time);
//                    let sleepy_time = epoch_time.duration_since(now);
                    let fifteen_minutes = time::Duration::from_secs(60 * 15);
                    thread::sleep(fifteen_minutes);
                },
                err => println!("{:?}", err),
            },
        }
    }
//    core.run(user::followers_ids(config.user_id, &config.token, &handle)
//        .map(|r| r.response)
//        .for_each(|id| { followers.insert(id); Ok(()) })).unwrap();
//
//    let reciprocals = friends.intersection(&followers).cloned().collect::<Vec<_>>();
//
//    println!("{} accounts that you follow follow you back.", reciprocals.len());
//
//    for user in core.run(user::lookup(&reciprocals, &config.token, &handle)).unwrap() {
//        println!("{} (@{})", user.name, user.screen_name);
//    }
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
