use std::{fmt::Display, path::Path, time::Instant};

use aoc::Parser;
use scraper::{Html, Selector};

pub fn base_url_for_day(day: i32) -> String {
    let year = dotenvy::var("AOC_YEAR").unwrap();
    format!("https://adventofcode.com/{year}/day/{day}")
}

pub fn run_and_upload<'a, C: Parser<'a>>(name: &str, input: &'static str) {
    println!("\nRunning challenge {}", name);

    let start = Instant::now();
    let challenge = C::must_parse(input);

    let file = Path::new("challenges").join(name).join("README.md");
    let readme = std::fs::read_to_string(file).expect("could not read file");
    let part_one = !readme.contains("--- Part Two ---");

    if part_one {
        let p1 = challenge.part_one();
        println!("took: {:?}", start.elapsed());
        println!("\tAnswer to part one: {p1}. ({:?})", start.elapsed());
        submit(name, 1, p1);
    } else {
        let p2 = challenge.part_two();
        println!("\tAnswer to part two: {p2}. ({:?})", start.elapsed());
        submit(name, 2, p2);
    }
}

fn submit(name: &str, level: usize, answer: impl Display) {
    let session = dotenvy::var("AOC_SESSION").unwrap();
    let day = name[3..].parse::<i32>().unwrap();
    let url = format!("{}/answer", base_url_for_day(day));

    let resp = ureq::post(&url)
        .set("Cookie", &format!("session={session}"))
        .send_form(&[
            ("level", &format!("{level}")),
            ("answer", &format!("{answer}")),
        ])
        .unwrap()
        .into_string()
        .unwrap();

    let html = Html::parse_document(&resp);
    let selector = Selector::parse("article span.day-success").unwrap();
    if html.select(&selector).count() > 0 {
        println!("Correct!");
    } else {
        println!("Wrong!");
        let file = Path::new("challenges").join(name).join("resp.html");
        std::fs::write(file, resp).unwrap();
    }
}

pub fn get_input(day: i32) -> String {
    let session = dotenvy::var("AOC_SESSION").unwrap();
    let url = format!("{}/input", base_url_for_day(day));

    ureq::get(&url)
        .set("Cookie", &format!("session={session}"))
        .send_bytes(&[])
        .unwrap()
        .into_string()
        .unwrap()
}

pub fn get_page_html(day: i32) -> String {
    let session = dotenvy::var("AOC_SESSION").unwrap();
    let url = base_url_for_day(day);

    ureq::get(&url)
        .set("Cookie", &format!("session={session}"))
        .send_bytes(&[])
        .unwrap()
        .into_string()
        .unwrap()
}
