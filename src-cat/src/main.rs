use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs;
use std::io::Write;
use std::io::{stdin, stdout};

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


fn main() -> std::io::Result<()> {
    let mn = nenter();
    let nlist = listout();
    let _ = fs::write("sheet.html",HTMLPRE);
    let mut f = File::options().append(true).open("sheet.html")?;
    for n in nlist.iter() {
        if n == "Intel" || n == "strategic" || n == "military" || n == "economic" {
            let t = webunsafe(n);
            writeln!(&mut f, r#"\n</table>\n<h2>{} Specialists</h2>\n<table class = "center">"#, t)?;
        } else {
            let t = webunsafe(&n);
            writeln!(&mut f, r#"<tr><td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}?generated_by=jeffnet_by_garbelia_used_by_{}">{}</a></p></td>"#, n, n, n, mn, t)?;
            writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=production?generated_by=jeffnet_by_garbelia_used_by_{}">Production</a></p></td>"#, n, n, n, mn)?;
            writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=production?generated_by=jeffnet_by_garbelia_used_by_{}">Production</a></p></td>"#, n, n, n, mn)?;
            writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=production?generated_by=jeffnet_by_garbelia_used_by_{}">Production</a></p></td>"#, n, n, n, mn)?;
            writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=production?generated_by=jeffnet_by_garbelia_used_by_{}">Production</a></p></td>"#, n, n, n, mn)?;
            writeln!(&mut f, r#"<td><p><a target="_blank" href="https://www.nationstates.net/container={}/nation={}/nation={}/page=nukes/view=production?generated_by=jeffnet_by_garbelia_used_by_{}">Production</a></p></td>"#, n, n, n, mn)?;
        }
    }
    writeln!(&mut f, "{}", HTMLSUF)?;
    Ok(())
}


fn nenter() -> String {
    let mut testn = String::new();
    print!("Please enter your main Nationstates nation: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut testn)
        .expect("Error!");
    if let Some('\n') = testn.chars().next_back() {
        testn.pop();
    }
    if let Some('\r') = testn.chars().next_back() {
        testn.pop();
    }
    return testn.parse::<String>().unwrap();
}

fn webunsafe(st: &str) -> String {
    let s = str::replace(&st, "_", " ");
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn listout() -> Vec<String> {
    let mut nl: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("nations.txt") {
        for line in lines.flatten() {
            println!("{}", &line);
            let websafea = str::replace(&line, " ", "_");
            let websafeb = websafea.to_lowercase();
            nl.push(websafeb);
            println!("{:?}", nl);
        }
    }
    return nl
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}