use std::{cell::RefCell, fmt::Display, path::Path, time::Instant};

use comrak::{
    nodes::{Ast, AstNode, NodeCode, NodeCodeBlock, NodeHeading, NodeLink, NodeList, NodeValue},
    Arena,
};
use scraper::{Html, Selector};
use url::Url;
use walkdir::WalkDir;

const YEAR: usize = 2023;

pub trait Parser: Sized + Challenge {
    fn parse(input: &'static str) -> nom::IResult<&'static str, Self>;
}

pub trait Challenge {
    const NAME: &'static str;

    fn part_one(self) -> impl Display;

    fn part_two(self) -> impl Display;
}

pub fn check<C: Parser + Clone>(input: &'static str) {
    let start = Instant::now();
    let challenge = C::parse(input).unwrap().1;
    let p1 = challenge.clone().part_one();
    let p2 = challenge.part_two();
    println!("took: {:?}", start.elapsed());

    println!("\tAnswer to part one: {p1}");
    println!("\tAnswer to part two: {p2}");
}

pub fn run<C: Parser>(input: &'static str) {
    println!("\nRunning challenge {}", C::NAME);

    let start = Instant::now();
    let challenge = C::parse(input).unwrap().1;

    let file = Path::new("challenges").join(C::NAME).join("README.md");
    let readme = std::fs::read_to_string(file).expect("could not read file");
    let part_one = !readme.contains("--- Part Two ---");

    if part_one {
        let p1 = challenge.part_one();
        println!("took: {:?}", start.elapsed());
        println!("\tAnswer to part one: {p1}. ({:?})", start.elapsed());
        submit::<C, _>(1, p1);
    } else {
        let p2 = challenge.part_two();
        println!("\tAnswer to part two: {p2}. ({:?})", start.elapsed());
        submit::<C, _>(2, p2);
    }
}

fn submit<C: Challenge, S: Display>(level: usize, answer: S) {
    let session = dotenvy::var("AOC_SESSION").unwrap();

    let day = C::NAME[3..].parse::<i32>().unwrap();
    let url = format!("https://adventofcode.com/{YEAR}/day/{day}/answer");

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
        let file = Path::new("challenges").join(C::NAME).join("resp.html");
        std::fs::write(file, resp).unwrap();
    }
}

pub fn create_project(day: i32) {
    let session = dotenvy::var("AOC_SESSION").unwrap();

    assert!(day >= 1);
    assert!(day <= 25);

    let project_name = format!("day{day:02}");
    let challenges = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("challenges");
    let template_path = challenges.join("day00");
    let path = challenges.join(&project_name);

    for entry in WalkDir::new(&template_path) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let rel = entry.path().strip_prefix(&template_path).unwrap();
            let out = path.join(rel);
            let content = fs_err::read_to_string(entry.path())
                .unwrap()
                .replace("day00", &project_name);
            fs_err::create_dir_all(out.parent().unwrap()).unwrap();
            fs_err::write(out, content).unwrap();
        }
    }

    let url = format!("https://adventofcode.com/{YEAR}/day/{day}/input");

    let data = ureq::get(&url)
        .set("Cookie", &format!("session={session}"))
        .send_bytes(&[])
        .unwrap()
        .into_string()
        .unwrap();

    fs_err::write(path.join("input.txt"), data).unwrap();
}

pub fn get_project_description(day: i32) {
    let session = dotenvy::var("AOC_SESSION").unwrap();

    assert!(day >= 1);
    assert!(day <= 25);

    let project_name = format!("day{day:02}");
    let challenges = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("challenges");
    let path = challenges.join(project_name);

    let url = format!("https://adventofcode.com/{YEAR}/day/{day}");

    let data = ureq::get(&url)
        .set("Cookie", &format!("session={session}"))
        .send_bytes(&[])
        .unwrap()
        .into_string()
        .unwrap();

    let html = Html::parse_document(&data);
    let selector = Selector::parse("article.day-desc").unwrap();

    let arena = Arena::new();
    let new_node =
        |nv: NodeValue| &*arena.alloc(AstNode::new(RefCell::new(Ast::new(nv, (0, 0).into()))));
    let mut document = new_node(NodeValue::Document);

    for elements in html.select(&selector) {
        for x in elements.traverse() {
            match x {
                ego_tree::iter::Edge::Open(x) => match x.value() {
                    scraper::Node::Text(t) => {
                        let mut b = document.data.borrow_mut();
                        match &mut b.value {
                            NodeValue::CodeBlock(c) => {
                                if !c.literal.is_empty() {
                                    c.literal += "\n";
                                }
                                c.literal += t.trim();
                            }
                            NodeValue::Code(c) => c.literal = t.trim().to_string(),
                            _ => {
                                drop(b);
                                document.append(new_node(NodeValue::Text(
                                    t.trim_matches(&['\n'] as &[char]).to_string(),
                                )));
                            }
                        }
                    }
                    scraper::Node::Element(e) if e.name() == "h2" => {
                        let section = new_node(NodeValue::Heading(NodeHeading {
                            level: 2,
                            setext: false,
                        }));
                        document.append(section);
                        document = section;
                    }
                    scraper::Node::Element(e) if e.name() == "p" => {
                        let section = new_node(NodeValue::Paragraph);
                        document.append(section);
                        document = section;
                    }
                    scraper::Node::Element(e) if e.name() == "em" => {
                        let b = document.data.borrow();
                        let mut re_add = None;

                        if let NodeValue::Code(_) = b.value {
                            drop(b);
                            re_add = Some(document);
                            let p = document.parent().unwrap();
                            document.detach();
                            document = p;
                        } else if let NodeValue::CodeBlock(_) = b.value {
                            continue;
                        }

                        let section = new_node(NodeValue::Strong);
                        document.append(section);
                        document = section;

                        if let Some(re_add) = re_add {
                            document.append(re_add);
                            document = re_add;
                        }
                    }
                    scraper::Node::Element(e) if e.name() == "code" => {
                        if x.parent().unwrap().value().as_element().unwrap().name() == "pre" {
                            let section = new_node(NodeValue::CodeBlock(NodeCodeBlock {
                                fenced: false,
                                fence_char: b'`',
                                fence_length: 3,
                                fence_offset: 0,
                                info: String::new(),
                                literal: String::new(),
                            }));
                            document.append(section);
                            document = section;
                        } else {
                            let section = new_node(NodeValue::Code(NodeCode {
                                num_backticks: 1,
                                literal: String::new(),
                            }));
                            document.append(section);
                            document = section;
                        }
                    }
                    scraper::Node::Element(e) if e.name() == "ul" => {
                        let section = new_node(NodeValue::List(NodeList {
                            list_type: comrak::nodes::ListType::Bullet,
                            marker_offset: 0,
                            padding: 0,
                            start: 0,
                            delimiter: comrak::nodes::ListDelimType::Paren,
                            bullet_char: b'-',
                            tight: false,
                        }));
                        document.append(section);
                        document = section;
                    }
                    scraper::Node::Element(e) if e.name() == "li" => {
                        let section = new_node(NodeValue::Item(NodeList {
                            list_type: comrak::nodes::ListType::Bullet,
                            marker_offset: 0,
                            padding: 0,
                            start: 0,
                            delimiter: comrak::nodes::ListDelimType::Paren,
                            bullet_char: b'-',
                            tight: false,
                        }));
                        document.append(section);
                        document = section;
                    }
                    scraper::Node::Element(e) if e.name() == "a" => {
                        let url = Url::parse(&url)
                            .unwrap()
                            .join(e.attr("href").unwrap())
                            .unwrap();
                        let section = new_node(NodeValue::Link(NodeLink {
                            url: url.to_string(),
                            title: String::new(),
                        }));
                        document.append(section);
                        document = section;
                    }
                    scraper::Node::Element(_) => {}
                    _ => {}
                },
                ego_tree::iter::Edge::Close(x) => match x.value() {
                    scraper::Node::Element(e) if e.name() == "h2" => {
                        document = document.parent().unwrap();
                    }
                    scraper::Node::Element(e) if e.name() == "p" => {
                        document = document.parent().unwrap();
                    }
                    scraper::Node::Element(e) if e.name() == "em" => {
                        if let NodeValue::CodeBlock(_) = document.data.borrow().value {
                            continue;
                        }
                        document = document.parent().unwrap();
                    }
                    scraper::Node::Element(e) if e.name() == "code" => {
                        document = document.parent().unwrap();
                    }
                    scraper::Node::Element(e) if e.name() == "ul" => {
                        document = document.parent().unwrap();
                    }
                    scraper::Node::Element(e) if e.name() == "li" => {
                        document = document.parent().unwrap();
                    }
                    scraper::Node::Element(e) if e.name() == "a" => {
                        document = document.parent().unwrap();
                    }
                    _ => {}
                },
            };
        }
    }

    let mut output = fs_err::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.join("README.md"))
        .unwrap();
    comrak::format_commonmark(document, &comrak::Options::default(), &mut output).unwrap();
}

#[test]
fn foo() {
    // let repo = gix::discover(".").unwrap();
    // let id = repo.head_tree_id().unwrap();
    // let tree = repo.find_object(id).unwrap().into_tree();

    // let challenges = Path::new(env!("CARGO_MANIFEST_DIR"))
    //     .parent()
    //     .unwrap()
    //     .join("challenges");
    // let template_path = challenges.join("day00").join("foo.txt");

    // let blob = repo.write_blob_stream(std::fs::File::open(template_path).unwrap()).unwrap();
    // // tree.

    create_project(8);
    get_project_description(8);
    // for i in 1..=25 {
    //     create_project(i);
    //     get_project_description(i);
    // }
}
