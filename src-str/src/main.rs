use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs;
use std::io::Write;
use scraper::{Html, Selector};
use std::{thread, time};
use getch_rs::{Getch, Key};

fn main() {
    let g = Getch::new();
    let nlist: Vec<String> = listout();
    let ratel = time::Duration::from_millis(1000);
    for n in nlist.iter() {
        let mut result = g.getch().unwrap();
        if result == Key::Alt('s') {
            scrape(n.clone());
        }
        let mut result = Key::EOF;
        loop {            
            match g.getch() {
                Ok(Key::Alt('s')) => continue,
                _ => break,
            }
        }
    }
}

fn scrape(n: String) {
    let n1 = "https://nationstates.net/nation=";
    let n2 = "/page=nukes?generated_by=jeffnet_by_garbelia_used_by_garbelia";
    let n5 = n1.to_string() + &n + n2;
    let response = reqwest::blocking::get(n5).unwrap().text().unwrap();
    
    let doc_body = Html::parse_document(&response);

    let strat = Selector::parse(".fancylike").unwrap();
        
    for strat in doc_body.select(&strat) {
        let spec = strat.text().collect::<Vec<_>>();
        println!("{:?}",spec)
    }
}

fn listout() -> Vec<String> {
    let mut nl: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("nations.txt") {
        for line in lines.flatten() {
            let websafea = str::replace(&line, " ", "_");
            let websafeb = websafea.to_lowercase();
            nl.push(websafeb);
        }
    }
    return nl
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}