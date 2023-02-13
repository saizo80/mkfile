use home::home_dir;
use std::{fs::File, io::Write};

fn main() {
    // get arguments
    let args: Vec<String> = std::env::args().collect();

    // drop the first argument (the program name)
    let args = &args[1..];

    let path = get_path(args[0].to_string());

    println!("Path: {}", path);

    let bytes = get_bytes(args[1].to_string());

    make_file(path, &bytes);
}

fn get_path(path: String) -> String {
    if !path.contains("/") && !path.contains("\\") {
        return format!(
            "{}/{}",
            std::env::current_dir().unwrap().to_str().unwrap(),
            path
        );
    } else if path.contains("~/") || path.contains("~\\") {
        // get home directory
        let home_dir = home_dir().unwrap();
        return path.replace("~", home_dir.to_str().unwrap());
    }
    path.to_string()
}

fn get_bytes(kilos: String) -> Vec<u8> {
    let kilos = kilos.parse::<u32>().unwrap();
    let bytes = kilos * 1024;

    // create an array [u8; bytes]
    let mut content = Vec::with_capacity(bytes as usize);

    for _ in 0..bytes {
        content.push(0);
    }
    content
}

fn make_file(path: String, content: &[u8]) {
    let mut file = File::create(path).unwrap();
    file.write_all(content).unwrap();
}
