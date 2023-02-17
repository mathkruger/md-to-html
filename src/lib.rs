use std::fs::{
    read_to_string,
    write
};
use std::env;
use wkhtmltopdf::*;

pub struct Args {
    pub input: String,
    pub output: String,
    pub style: String,
    pub format: String
}

impl Args {
    pub fn get() -> Args {
        let args: Vec<String> = env::args().collect();

        Args {
            input: args[1].clone(),
            output: args[2].clone(),
            style: if args.len() >= 4 {
                args[3].clone()
            } else {
                "dark".to_owned()
            },
            format: if args.len() >= 5 {
                args[4].clone()
            } else {
                "html".to_owned()
            }
        }
    }
}

pub fn read_file(filename: &str) -> String {
    let file: String = match read_to_string(filename) {
        Ok(value) => value,
        Err(_) => String::new()
    };

    file
}

pub fn write_file(path: &str, contents: String) -> () {
    match write(path, contents) {
        Ok(_) => (),
        Err(_) => print!("There was an error to write the file")
    }
}

pub fn save_html_as_pdf(html: &String, title: &String, filename: &String) {
    let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    
    let mut pdfout = pdf_app.builder()
        .orientation(Orientation::Landscape)
        .margin(Size::Inches(2))
        .title(title)
        .build_from_html(&html)
        .expect("failed to build pdf");

    pdfout.save(filename).expect(format!("failed to save {filename}").as_str());
}
