use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;
use text_io::read;
use std::fs;

const HTMLPRE: &str = r#"<!doctype html>
<html>
<head>
<style>
td.createcol p {
	padding-left: 10em;
}

p{
	font-family: 'Segoe UI';
}
a {
	text-decoration: none;
    font-family: 'Segoe UI';
	color: black;
}

a:visited {
	color: grey;
}

table {
	border-collapse: collapse;
	max-width: 1000%;
	border: 1px solid grey;
}

table.center {
    margin-left:auto; 
    margin-right:auto;
}

tr, td {
	border-bottom: 2px solid green;
}

tr.end, td.end {
	border-bottom: 5px solid rgb(216, 216, 32);
}

td p {
	padding: 0.7em;
}

body {
	text-align:center;
	padding-top: 50px;
	margin:0;
}

tr:hover {
	background-color: lightgrey;
}

.sticky {
  position: fixed;
  top: 0;
  width: 100%;
  background-color: #CCCD;
  border-radius: 0px 0px 5px 5px;
}

</style>
</head>

<table class = "center">"#;

const HTMLSUF: &str = r#"
</table>
</body>
</html>"#;

#[derive(Debug, PartialEq)]
enum Classification {
    Strategic,
    Cleanup,
    Military,
    Economic,
}

#[derive(Debug)]
struct Nation {
    n: String,
    t: Classification,
}

fn main() -> std::io::Result<()> {
    let mn = nenter();
    let nlist = listout();
    let mut intelv = Vec::new();
    let mut stratv = Vec::new();
    let mut milv = Vec::new();
    let mut ecov = Vec::new();
    match nlist {
        Ok(vec) => {
            println!("{:?}", vec[0]);
            for n in vec.iter() {
                match n.t {
                    Classification::Cleanup => intelv.push(n.n.clone()),
                    Classification::Strategic => stratv.push(n.n.clone()),
                    Classification::Military => milv.push(n.n.clone()),
                    Classification::Economic => ecov.push(n.n.clone()),
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    let _ = fs::write("jnday_sheet.html", HTMLPRE);
    let mut f = File::options().append(true).open("jnday_sheet.html")?;
//    for i in nlist() {
//        match i.t {
//            Classification::Cleanup => intelv.push(n.n),
//            Classification::Strategic => stratv.push(n.n),
//            Classification::Military => milv.push(n.n),
//            Classification::Economic => ecov.push(i.n),
//        }
//    }
    writeln!(
        &mut f,
        r#"<h2>Strategic Specialists</h2><table id = "strat" class = "center">"#
    )?;
    for n in stratv.iter() {
        let t = webunsafe(&n);
        writeln!(
            &mut f,
            r#"<tr><td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}?generated_by=jeffnet_by_garbelia_used_by_{}">{}</a></p></td>"#,
            n, n, n, mn, t
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=production?generated_by=jeffnet_by_garbelia_used_by_{}">production</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=1/view=incoming?generated_by=jeffnet_by_garbelia_used_by_{}">incoming</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=95/view=nations/start=0?generated_by=jeffnet_by_garbelia_used_by_{}">target</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=targets?generated_by=jeffnet_by_garbelia_used_by_{}">launch</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=0?consider_join_faction=1&join_faction=1?generated_by=jeffnet_by_garbelia_used_by_{}">join</a></p></td>"#,
            n, n, n, mn
        )?;
    }
    writeln!(
        &mut f,
        r#"</table><h2>Economic Specialists</h2><table id = "econ" class = "center">"#
    )?;
    for n in ecov.iter() {
        let t = webunsafe(&n);
        writeln!(
            &mut f,
            r#"<tr><td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}?generated_by=jeffnet_by_garbelia_used_by_{}">{}</a></p></td>"#,
            n, n, n, mn, t
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=production?generated_by=jeffnet_by_garbelia_used_by_{}">production</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=1/view=incoming?generated_by=jeffnet_by_garbelia_used_by_{}">incoming</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=95/view=nations/start=0?generated_by=jeffnet_by_garbelia_used_by_{}">target</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=targets?generated_by=jeffnet_by_garbelia_used_by_{}">launch</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=0?consider_join_faction=1&join_faction=1?generated_by=jeffnet_by_garbelia_used_by_{}">join</a></p></td>"#,
            n, n, n, mn
        )?;
    }
    writeln!(
        &mut f,
        r#"</table><h2>Cleanup Specialists</h2><table id = "intel" class = "center">"#
    )?;
    for n in intelv.iter() {
        let t = webunsafe(&n);
        writeln!(
            &mut f,
            r#"<tr><td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}?generated_by=jeffnet_by_garbelia_used_by_{}">{}</a></p></td>"#,
            n, n, n, mn, t
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=production?generated_by=jeffnet_by_garbelia_used_by_{}">production</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=1/view=incoming?generated_by=jeffnet_by_garbelia_used_by_{}">incoming</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=95/view=nations/start=0?generated_by=jeffnet_by_garbelia_used_by_{}">target</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=targets?generated_by=jeffnet_by_garbelia_used_by_{}">launch</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=0?consider_join_faction=1&join_faction=1?generated_by=jeffnet_by_garbelia_used_by_{}">join</a></p></td>"#,
            n, n, n, mn
        )?;
    }
    writeln!(
        &mut f,
        r#"</table><h2>Military Specialists</h2><table id = "mil" class = "center">"#
    )?;
    for n in milv.iter() {
        let t = webunsafe(&n);
        writeln!(
            &mut f,
            r#"<tr><td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}?generated_by=jeffnet_by_garbelia_used_by_{}">{}</a></p></td>"#,
            n, n, n, mn, t
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=production?generated_by=jeffnet_by_garbelia_used_by_{}">production</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=1/view=incoming?generated_by=jeffnet_by_garbelia_used_by_{}">incoming</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=95/view=nations/start=0?generated_by=jeffnet_by_garbelia_used_by_{}">target</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=targets?generated_by=jeffnet_by_garbelia_used_by_{}">launch</a></p></td>"#,
            n, n, n, mn
        )?;
        writeln!(
            &mut f,
            r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=0?consider_join_faction=1&join_faction=1?generated_by=jeffnet_by_garbelia_used_by_{}">join</a></p></td>"#,
            n, n, n, mn
        )?;
    }
    writeln!(&mut f, "</table>")?;
    writeln!(&mut f, "{}", HTMLSUF)?;
    Ok(())
}

fn nenter() -> String {
    print!("Please enter your main Nationstates nation: ");
    let nation: String = read!();
    return nation;
}

fn webunsafe(st: &String) -> String {
    let s = str::replace(&st, "_", " ");
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn listout() -> Result<Vec<Nation>, Box<dyn Error>> {
    let file_path = "nations.csv";
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path);
    let mut nl: Vec<Nation> = Vec::new();
    for result in rdr?.records() {
        let record = result?;
        println!("{:?}", record);
        let vconv = match &record[1] {
                "Strategic Specialist" => Classification::Strategic,
                "Cleanup Specialist" => Classification::Cleanup,
                "Military Specialist" => Classification::Military,
                "Economic Specialist" => Classification::Economic,
                _ => panic!("The contents of nations.txt are irregular!"),
            };
        let entry = Nation {
            n: record[0].to_string(),
            t: vconv,
        };
        nl.push(entry);
    }
    Ok(nl)
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
