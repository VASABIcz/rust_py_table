// use proc_macro::bridge::PanicMessage::String;
use pyo3::prelude::*;

fn teet(desc: Vec<Vec<&str>>) -> String {
    let mut max = vec![];
    let mut result = String::new();

    for _ in &desc[0] {
        max.push(0);
    }

    for row in &desc {
        for item in row.iter().enumerate() {
            if max[item.0] < item.1.to_string().len() {
                max[item.0] = item.1.to_string().len()
            }
        }
    }

    let mut whole_line = String::from("+");

    for m in &max {
        whole_line.push_str("-".repeat(m+2).as_str());
        whole_line.push('+')
    }


    // println!("{}", &whole_line);
    result.push_str(&whole_line);

    for row in desc {
        let mut lin = String::from("|");

        for item in row.iter().enumerate() {
            let lenn = &max[item.0]-item.1.len();

            lin.push(' ');
            lin.push_str(item.1);
            lin.push_str(" ".repeat(lenn).as_str());
            lin.push(' ');
            lin.push('|');
        }
        result.push_str(&lin);
        result.push_str(&whole_line);
    }
    result
}

fn main() {
    let row = vec!["aaa", " b", "ccccccccccccccc", "EEEEEEEEEEEEE E E E E"];
    let row2 = vec!["a", " a", "a", "a"];
    let lines = vec![row.clone(), row2.clone(), row.clone(), row2.clone(), row.clone()];


    let res = teet(lines);
    println!("{}", res);
}
