// use std::env;
use std::path::Path;
use std::time::SystemTime;
use pdf_extract::{extract_text, extract_text_from_mem};
use whichlang::{detect_language,Lang};
// fn main() {
//     let mut args = env::args();
//     let _exe_name = args.next().unwrap();
//     let pdffile = args.next().expect("First argument is pdffile");
//     let outputfile = args.next().unwrap_or_else(|| pdffile.replace(".pdf", ".txt"));


//     let now = SystemTime::now();
//     println!("Transforming {pdffile} into {outputfile}");
//     let path = Path::new(&pdffile);
    
//     let doc ;
//     let lang;
//     (doc,lang) = pdf2text(path);
// } 

fn pdf2text_path(path: &Path) -> (String,Lang) {

    let now = SystemTime::now();

    let doc = extract_text(path).unwrap();
    let time_passed: u128 = now.elapsed().unwrap().as_millis();
    let lang = detect_language(&doc);
    let time_passed_lang = now.elapsed().unwrap().as_millis() - time_passed;
    
    println!("{doc}");
    println!("Time passed: {} ms",time_passed);
    println!("Language: {}",lang.three_letter_code());
    println!("Time passed: {} ms",time_passed_lang);

    (doc,lang)

}


pub fn pdf2text(bytes: &Vec<u8>) -> Result<(String,Lang),&str> {


    match extract_text_from_mem(bytes){
        Ok(doc) => {
            let lang = detect_language(&doc);
            Ok((doc,lang))
        }
        Err(err) => {
            let error = match err {
                pdf_extract::OutputError::FormatError(_) => "Format Error",
                pdf_extract::OutputError::IoError(_) => "Format Error",
                pdf_extract::OutputError::PdfError(_) => "Pdf Error",
            };
            Err(error)
        }

    }
}