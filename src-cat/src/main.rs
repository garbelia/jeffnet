use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs;
use std::io::Write;
use text_io::read;

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

#[derive(Debug)]
#[derive(PartialEq)]
enum Classification {
    Strategic,
    Intel,
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
    let _ = fs::write("jnday_sheet.html",HTMLPRE);
    let mut f = File::options().append(true).open("jnday_sheet.html")?;
    let mut intelv = Vec::new();
    let mut stratv = Vec::new();
    let mut milv = Vec::new();
    let mut ecov = Vec::new();
    for n in nlist.iter() {
        match n.t {
            Classification::Cleanup => intelv.push(n.n.clone()),
            Classification::Strategic => stratv.push(n.n.clone()),
            Classification::Military => milv.push(n.n.clone()),
            Classification::Economic => ecov.push(n.n.clone()),
        }
    }
    writeln!(&mut f, r#"<h2>Strategic Specialists</h2><table id = "strat" class = "center">"#)?;
    for i in stratv.iter() {
        let n = str::replace(&i, r#"""#, "");
        let t = webunsafe(&n);
        writeln!(&mut f, r#"<tr><td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}?generated_by=jeffnet_by_garbelia_used_by_{}">{}</a></p></td>"#, n, n, n, mn, t)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=production?generated_by=jeffnet_by_garbelia_used_by_{}">production</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=1/view=incoming?generated_by=jeffnet_by_garbelia_used_by_{}">incoming</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=95/view=nations/start=0?generated_by=jeffnet_by_garbelia_used_by_{}">target</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=targets?generated_by=jeffnet_by_garbelia_used_by_{}">launch</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=0?consider_join_faction=1&join_faction=1?generated_by=jeffnet_by_garbelia_used_by_{}">join</a></p></td>"#, n, n, n, mn)?;    }
    writeln!(&mut f, r#"</table><h2>Economic Specialists</h2><table id = "econ" class = "center">"#)?;
    for i in ecov.iter() {
        let n = str::replace(&i, r#"""#, "");
        let t = webunsafe(&n);
        writeln!(&mut f, r#"<tr><td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}?generated_by=jeffnet_by_garbelia_used_by_{}">{}</a></p></td>"#, n, n, n, mn, t)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=production?generated_by=jeffnet_by_garbelia_used_by_{}">production</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=1/view=incoming?generated_by=jeffnet_by_garbelia_used_by_{}">incoming</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=95/view=nations/start=0?generated_by=jeffnet_by_garbelia_used_by_{}">target</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=targets?generated_by=jeffnet_by_garbelia_used_by_{}">launch</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=0?consider_join_faction=1&join_faction=1?generated_by=jeffnet_by_garbelia_used_by_{}">join</a></p></td>"#, n, n, n, mn)?;
    }
    writeln!(&mut f, r#"</table><h2>Cleanup Specialists</h2><table id = "intel" class = "center">"#)?;
    for i in intelv.iter() {
        let n = str::replace(&i, r#"""#, "");
        let t = webunsafe(&n);
        writeln!(&mut f, r#"<tr><td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}?generated_by=jeffnet_by_garbelia_used_by_{}">{}</a></p></td>"#, n, n, n, mn, t)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=production?generated_by=jeffnet_by_garbelia_used_by_{}">production</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=1/view=incoming?generated_by=jeffnet_by_garbelia_used_by_{}">incoming</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=95/view=nations/start=0?generated_by=jeffnet_by_garbelia_used_by_{}">target</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=targets?generated_by=jeffnet_by_garbelia_used_by_{}">launch</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=0?consider_join_faction=1&join_faction=1?generated_by=jeffnet_by_garbelia_used_by_{}">join</a></p></td>"#, n, n, n, mn)?;    }
    writeln!(&mut f, r#"</table><h2>Military Specialists</h2><table id = "mil" class = "center">"#)?;
    for i in milv.iter() {
        let n = str::replace(&i, r#"""#, "");
        let t = webunsafe(&n);
        writeln!(&mut f, r#"<tr><td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}?generated_by=jeffnet_by_garbelia_used_by_{}">{}</a></p></td>"#, n, n, n, mn, t)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=production?generated_by=jeffnet_by_garbelia_used_by_{}">production</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=1/view=incoming?generated_by=jeffnet_by_garbelia_used_by_{}">incoming</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=95/view=nations/start=0?generated_by=jeffnet_by_garbelia_used_by_{}">target</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=targets?generated_by=jeffnet_by_garbelia_used_by_{}">launch</a></p></td>"#, n, n, n, mn)?;
        writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=faction/fid=0?consider_join_faction=1&join_faction=1?generated_by=jeffnet_by_garbelia_used_by_{}">join</a></p></td>"#, n, n, n, mn)?;    }
    writeln!(&mut f, "</table>")?;
    writeln!(&mut f, "{}", HTMLSUF)?;
    Ok(())
}


fn nenter() -> String {
    print!("Please enter your main Nationstates nation: ");
    let nation: String = read!();
    return nation
}

fn webunsafe(st: &String) -> String {
    let s = str::replace(&st, "_", " ");
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn listout () -> Vec<Nation> {
    let mut nl: Vec<Nation> = Vec::new();
    if let Ok(lines) = read_lines("nations.txt") {
        for line in lines.flatten() {
            let cutter = str::replace(&line, "(", "");
            let cut = str::replace(&cutter, ")", "");
            let v: Vec<&str> = cut.split(",").collect();
            let _r = v[0].to_string();
            let p = v[1];
            let websafea = str::replace(&v[0], " ", "_");
            let websafeb = websafea.to_lowercase();
            let vconv = match p {
                r#" "Strategic Specialist""# => Classification::Strategic,
                r#" "Cleanup Specialist""# => Classification::Cleanup,
                r#" "Military Specialist""# => Classification::Military,
                r#" "Economic Specialist""# => Classification::Economic,
                _ => panic!("The contents of nations.txt are irregular!"),
            };
            let finalo = Nation {n:websafeb,t:vconv};
            nl.push(finalo);
        }
    }
    return nl
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}