use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs;
use std::io::Write;
use scraper::{Html, Selector};
use getch_rs::{Getch, Key};

fn main() -> std::io::Result<()> {
    println!("Press Alt+S to process a nation, and any other key to move to the next in the list.");
    let g = Getch::new();
    let nlist: Vec<String> = listout();
    let mut tlist: Vec<(String,String)> = Vec::new();
    for n in nlist.iter() {
        let result = g.getch().unwrap();
        if result == Key::Char('s') {
            tlist.push(scrape(n.clone()));
            println!("{:?}",tlist)
        }
        loop {            
            match g.getch() {
                Ok(Key::Char('s')) => continue,
                _ => break,
            }
        }
    }
    let _ = fs::write("SORTEDnations.txt", "");
    let mut f = File::options().append(true).open("SORTEDnations.txt")?;
    for n in tlist.iter() {
        writeln!(&mut f, "{:?}", n)?;
    }
    Ok(())
}

fn scrape(n: String) -> (String, String) {
    let n1 = "https://nationstates.net/nation=";
    let n2 = "/page=nukes?generated_by=jeffnet_by_garbelia_used_by_garbelia";
    let n5 = n1.to_string() + &n + n2;
    let response = reqwest::blocking::get(n5).unwrap().text().unwrap();
    let doc_body = Html::parse_document(&response);
    let strat = Selector::parse(".fancylike").unwrap();
    let mut specs: String = String::new(); 
        for strat in doc_body.select(&strat) {
            let spec = strat.text().collect::<Vec<_>>();
            if let Some(first_spec) = spec.get(0) {
                specs = first_spec.to_string();
            }
    }
    println!("Nation {} has been processed.",n);
    let nnlist = (n.clone(), specs);
    nnlist
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