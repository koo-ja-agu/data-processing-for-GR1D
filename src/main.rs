use std::fs;
use std::io::{BufRead, BufReader, Write};

use std::str::FromStr;



fn main() {
    let filename = "./data/original_data";
    output(load_data(filename).unwrap());
}

fn load_data(filename: &str) -> std::io::Result<Vec<Vec<f64>>> {
    let mut data = Vec::new();
    let mut l = 0;

    for result in BufReader::new(fs::File::open(filename)?).lines().skip(2) {
        let line = Vec::new();
        data.push(line);
        for s in result?.split_whitespace(){
            let l_s = match f64::from_str(s) {
                Ok(value) => value,
                Err(_err) => 0.0,
            };
            data[l].push(l_s);
        }
        l += 1;
    } 
    Ok(data)
}

fn output(data: Vec<Vec<f64>>) {
    let s = format!("./data/output");
    let mut buffer = fs::File::create(s).unwrap();
    let mut l = 1;
    for r in &data {
        write!(buffer,"{}\t\t",l);      //grid
        write!(buffer,"{:.012E}\t\t",r[1]);   //cell outer total mass
        write!(buffer,"{:.012E}\t\t",r[2]);   //cell outer radius
        write!(buffer,"{:.012E}\t\t",r[5]);   //cell temperture
        write!(buffer,"{:.012E}\t\t",r[4]);   //cell density
        write!(buffer,"{:.012E}\t\t",r[3]);   //cell outer velocity
        write!(buffer,"{:.012E}\n",r[10]);  //cell A_bar
        l += 1;
    }
}
