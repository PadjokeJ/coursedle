use std::{collections::HashMap, fs::File, io::*, rc::Rc};

use reqwest::{self, Response, blocking::get};
use serde::{Deserialize, Serialize};
use soup::prelude::*;

use markup5ever::rcdom::Node;

use regex::regex;

const PRE: &str = "https://edu.epfl.ch";

const CMS: &str = "/studyplan/en/cms/";
const PROP: &str = "/studyplan/en/propedeutics/";
const BACH: &str = "/studyplan/en/bachelor/";
const MAST: &str = "/studyplan/en/master/";

#[derive(Serialize, Deserialize, Debug)]
enum Lang {
    Fr,
    En,
    De,
    FrEn,
}

impl Lang {
    fn from_string(s: String) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "frfr" => Some(Self::Fr),
            "enen" => Some(Self::En),
            "dede" => Some(Self::De),
            "fr/enfr/en" => Some(Self::FrEn),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Session {
    Winter,
    Summer,
}

impl Session {
    fn from_string(s: &String) -> Self {
        if regex!("(S|s)ummer").is_match(s) {
            Self::Summer
        } else {
            Self::Winter
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Semester {
    CMS,
    MAN,
    BA1,
    BA2,
    BA3,
    BA4,
    BA5,
    BA6,
    MA1,
    MA2,
    MP1,
    MP2,
    ANY,
}

impl Semester {
    fn find_string(n: &Rc<Node>) -> String {
        let d = n
            .tag("div")
            .find_all()
            .filter(|n| {
                n.get("class").unwrap().starts_with("bachlor")
                    && !n
                        .children()
                        .next()
                        .unwrap()
                        .get("class")
                        .unwrap()
                        .eq("schedule-text no-course")
            })
            .next();

        match d {
            None => "".to_string(),
            Some(d) => d.get("data-title").unwrap(),
        }
    }
    fn from_string(s: String) -> Self {
        match s.as_str() {
            "Mise à niveau" => Self::MAN,
            "Bachelor 1" => Self::BA1,
            "Bachelor 2" => Self::BA2,
            "Bachelor 3" => Self::BA3,
            "Bachelor 4" => Self::BA4,
            "Bachelor 5" => Self::BA5,
            "Bachelor 6" => Self::BA6,
            "Master 1" => Self::MA1,
            "Master 2" => Self::MA2,
            "MP Spring" => Self::MP1,
            "MP Autumn" => Self::MP2,
            _ => Self::ANY,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Course {
    title: String,
    link: String,
    course_id: String,
    profs: Vec<String>,
    language: Lang,
    credits: i32,
    session: Session,
    semester: Semester,
    section: String,
}

#[derive(Serialize, Deserialize)]
struct AllData {
    courses: HashMap<String, Course>,
}

async fn get_sections(link: &str) -> Vec<String> {
    let soup = Soup::new(get_link(link).await.as_str());

    let cont = soup.tag("main").find().unwrap();
    cont.tag("a")
        .find_all()
        .map(|a| a.get("href").unwrap())
        .collect::<Vec<_>>()
}

async fn get_courses(link: &str) -> HashMap<String, Course> {
    let soup = Soup::new(get_link(link).await.as_str());

    let cont = soup.tag("main").find().unwrap();
    cont.tag("div")
        .find_all()
        .filter(|n| n.get("class").unwrap().eq("line-down"))
        .map(|n| n.children().collect::<Vec<_>>()[0].clone())
        .map(|n| Course {
            title: n
                .tag("div")
                .find_all()
                .filter(|n| match n.get("class") {
                    None => false,
                    Some(n) => n.eq("cours-name"),
                }).filter_map(|n| n.tag("a").find())
                .fold(String::new(), |a, x| format!("{}{}", a, x.text())),
            link:  n
                .tag("div")
                .find_all()
                .filter(|n| match n.get("class") {
                    None => false,
                    Some(n) => n.eq("cours-name"),
                }).filter_map(|n| n.tag("a").find())
                .map(|n| n.get("href").unwrap())
                .fold(String::new(), |a, x| format!("{}{}", a, x)),
            course_id: get_data(&n, "cours-info")
                .split(" / ")
                .nth(0)
                .unwrap()
                .to_string(),
            profs: n
                .tag("div")
                .find_all()
                .filter(|n| n.get("class").unwrap().eq("enseignement-name"))
                .collect::<Vec<_>>()[0]
                .tag("a")
                .find_all()
                .map(|a| a.text())
                .collect::<Vec<_>>(),
            language: Lang::from_string(get_data(&n, "langue")).unwrap(),
            credits: match i32::from_str_radix(get_data(&n, "credit-time").as_str(), 10u32) {
                Ok(i) => i,
                Err(_) => 0,
            },
            session: Session::from_string(&get_data(&n, "exam-text")),
            semester: Semester::from_string(Semester::find_string(&n)),
            section: get_data(&n, "cours-info")
                .split(" / ")
                .nth(1)
                .unwrap()
                .to_string(),
        })
        .filter(|c| !c.course_id.is_empty() && !c.title.is_empty())
        .map(|c| (c.course_id.clone(), c))
        .collect::<HashMap<String, Course>>()
}

async fn get_link(link: &str) -> String {
    let mut s = String::new();
    s.push_str(PRE);
    s.push_str(link);

    let res = reqwest::get(s).await.unwrap();
    res.text().await.unwrap()
}

fn get_data(node: &Rc<Node>, clazz: &str) -> String {
    node.tag("div")
        .find_all()
        .filter(|n| match n.get("class") {
            None => false,
            Some(n) => n.eq(clazz),
        })
        .fold(String::new(), |a, x| format!("{}{}", a, x.text()))
}

async fn parse() {
    let paths = [/*CMS, */ PROP, BACH, MAST];
    let mut hm = HashMap::new();
    for i in paths {
        let sections = get_sections(i).await;

        for i in sections {
            let courses = get_courses(&i).await;
            hm.extend(courses);
        }
    }
    let mut output = File::create("data.json").unwrap();
    write!(
        output,
        "{}",
        serde_json::to_string(&AllData { courses: hm }).unwrap()
    )
    .unwrap();
}

#[tokio::main]
async fn main() {
    parse().await;
}
