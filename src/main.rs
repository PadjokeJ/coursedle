use std::rc::Rc;

use reqwest::{self, Response};
use serde::{Deserialize, Serialize};
use soup::prelude::*;

use markup5ever::rcdom::Node;

use regex::regex;

const PRE: &str = "https://edu.epfl.ch";

const CMS: &str = "/studyplan/en/cms/";
const PROP: &str = "/studyplan/en/propedeutics/";
const BACH: &str = "/studyplan/en/bachelor/";
const MAST: &str = "/studyplan/en/master/";

#[derive(Serialize, Deserialize)]
enum Lang {
    Fr,
    En,
    De,
}

impl Lang {
    fn from_string(s: String) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "fr" => Some(Self::Fr),
            "en" => Some(Self::En),
            "de" => Some(Self::De),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Session {
    Winter,
    Summer,
}

impl Session {
    fn from_string(s: &String) -> Self {
        if regex!("(S|s)ummer").is_match(s) { Self::Summer } else { Self::Winter }
    }
}

#[derive(Serialize, Deserialize)]
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
}

#[derive(Serialize, Deserialize)]
struct Course {
    title: String,
    link: String,
    course_id: String,
    profs: Vec<String>,
    language: Lang,
    credits: i32,
    session: Session,
    semester: Semester,
}

async fn get_sections(link: &str) -> Vec<String> {
    let soup = Soup::new(get_link(link).await.as_str());

    let cont = soup.tag("main").find().unwrap();
    cont.tag("a")
        .find_all()
        .map(|a| a.get("href").unwrap())
        .collect::<Vec<_>>()
}

async fn get_courses(link: &str) -> Vec<Course> {
    let soup = Soup::new(get_link(link).await.as_str());

    let cont = soup.tag("main").find().unwrap();
    cont.tag("div")
        .find_all()
        .filter(|n| n.get("class").unwrap().eq("line-down"))
        .map(|n| n.children().collect::<Vec<_>>()[0].clone())
        .map(|n| Course {
            title: get_data(&n, "cours-name"),
            link: get_data(&n, "cours-name"),
            course_id: get_data(&n, "cours-info"),
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
            credits: i32::from_str_radix(get_data(&n, "credit-time").as_str(), 10u32).unwrap(),
            session: Session::from_string(&get_data(&n, "exam-text")),
            semester: todo!(),
        })
        .collect::<Vec<_>>()
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
        .filter(|n| n.get(clazz).unwrap().eq("cours"))
        .fold(String::new(), |a, x| format!("{}{}", a, x.text()))
}

#[tokio::main]
async fn main() {
    let paths = [CMS, PROP, BACH, MAST];
    for i in paths {
        let sections = get_sections(i).await;
        for i in sections {
            println!("{}", i);
        }
    }
}
