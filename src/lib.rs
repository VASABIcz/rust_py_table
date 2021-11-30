//use proc_macro::bridge::PanicMessage::String;
use pyo3::prelude::*;


#[pyfunction]
fn draw(desc: Vec<Vec<&str>>) -> PyResult<Vec<String>> {
    let mut max = vec![];
    // let mut result = String::new();
    let mut res = vec![];

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
    // result.push_str(&whole_line);
    res.push(whole_line.clone());
    // result.push_str("\n");

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
        // result.push_str(&lin);
        // result.push_str("\n");
        // result.push_str(&whole_line);
        // result.push_str("\n");

        res.push(lin);
        res.push(whole_line.clone());
    }
    Ok(res)
}


#[pymodule]
fn table(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(draw, m)?)?;

    Ok(())
}
