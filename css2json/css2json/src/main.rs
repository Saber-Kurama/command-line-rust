use std::fs::File;
use std::io::{ BufReader, BufRead, Write};

    // let mut file = File::open("path_to_your_file").expect("Unable to open the file");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).expect("Unable to read the file");

fn css2json(css_file_path: &str) {
    let file = File::open(css_file_path).expect("没有该文件");
    let mut contents = String::new();
    // file.read_to_string(&mut contents).expect("无法读取该文件");
    let reader = BufReader::new(file);
    contents.push_str("{\n");
    for line in reader.lines() {
        let line = line.expect("无法读取行");
        if line.contains(":") {
            let v: Vec<&str> = line.split(':').collect();
            let key = v[0].trim();
            let value = v[1].trim_end_matches(';').trim();
            contents.push_str(&format!("\"{}\": \"{}\",", key, value));
            contents.push_str("");
            contents.push('\n');
        }
    }
    contents.push_str("}");
    // 
    let mut file = File::create("output.json").expect("无法创建文件");
    file.write_all(contents.as_bytes()).expect("无法写入文件");
    println!("{}", contents)
}

fn main() {
    let css_file = "./files/a.css";
    css2json(css_file); 
    println!("Hello, world!");
}
