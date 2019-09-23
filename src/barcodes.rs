extern crate barcoders;
extern crate qrcode_generator;

use qrcode_generator::QrCodeEcc;
use barcoders::sym::codabar::*;
use barcoders::sym::code11::*;
use barcoders::generators::image::*;
use barcoders::sym::code39::*;
use barcoders::sym::code93::*;
use barcoders::sym::ean8::*;
use barcoders::sym::ean13::*;
use chrono::DateTime;
use chrono::Utc;
use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use std::io::Write;


// Function gets the date to timestamp barcodes
pub fn get_fmt_date()->String{
    let utc: DateTime<Utc> = Utc::now();
    let formatted_date = utc.format("%d-%m-%y %H:%M:%S").to_string();
    return formatted_date;
}

//function that will generate the file path
// When supplied a codeType &str
fn construct_file_name(code_type: &str)-> String{
    let code = String::from(code_type);
    let date = get_fmt_date();

    let mut file_name= String::from("src/resources/generated/");
    file_name.push_str(&code);
    file_name.push_str("/");
    file_name.push_str(&code);
    file_name.push_str("-");
    file_name.push_str(&date);
    file_name.push_str(".png");
    return file_name;

}


//text input Params [A-D0-9]
pub fn create_codabar_png(text: &str){

    let barcode =Codabar::new(text).unwrap();
    let png  = Image::png(80);
    let encoded = barcode.encode();

    // generators return a Result<Vec<u8>, barcoders::error::Error) of encoded bytes
    let bytes = png.generate(&encoded[..]).unwrap();

    let created_file_path = construct_file_name("codabar");

    let file = File::create(&Path::new(&created_file_path)).unwrap();

    let mut writer = BufWriter::new(file);
    writer.write(&bytes[..]).unwrap();
}

//text input Params [A-Z0-9] and  (-, ., $, /, +, %, and space)
pub fn create_code11_png(text: &str){

    let barcode =Code11::new(text).unwrap();
    let png  = Image::png(80);
    let encoded = barcode.encode();

    let bytes = png.generate(&encoded[..]).unwrap();

    let created_file_path = construct_file_name("code11");

    let file = File::create(&Path::new(&created_file_path)).unwrap();

    let mut writer = BufWriter::new(file);
    writer.write(&bytes[..]).unwrap();
}

//text input Params [A-Z0-9] and  (-, ., $, /, +, %, and space)
pub fn create_code39_png(text: &str){

    let barcode =Code39::new(text).unwrap();
    let png  = Image::png(80);
    let encoded = barcode.encode();

    // generators return a Result<Vec<u8>, barcoders::error::Error) of encoded bytes
    let bytes = png.generate(&encoded[..]).unwrap();

    let created_file_path = construct_file_name("code39");

    let file = File::create(&Path::new(&created_file_path)).unwrap();

    let mut writer = BufWriter::new(file);
    writer.write(&bytes[..]).unwrap();
}

//text input Params [A-Z0-9] and  (-, ., $, /, +, %, and space)
pub fn create_code93_png(text: &str){

    let barcode =Code93::new(text).unwrap();
    let png  = Image::png(80);
    let encoded = barcode.encode();

    // generators return a Result<Vec<u8>, barcoders::error::Error) of encoded bytes
    let bytes = png.generate(&encoded[..]).unwrap();

    let created_file_path = construct_file_name("code93");

    let file = File::create(&Path::new(&created_file_path)).unwrap();

    let mut writer = BufWriter::new(file);
    writer.write(&bytes[..]).unwrap();
}

// 8[digits]
pub fn create_ean8_png(text: &str){

    let barcode =EAN8::new(text).unwrap();
    let png  = Image::png(80);
    let encoded = barcode.encode();

    // generators return a Result<Vec<u8>, barcoders::error::Error) of encoded bytes
    let bytes = png.generate(&encoded[..]).unwrap();

    let created_file_path = construct_file_name("ean8");

    let file = File::create(&Path::new(&created_file_path)).unwrap();

    let mut writer = BufWriter::new(file);
    writer.write(&bytes[..]).unwrap();
}

// 13 digits
pub fn create_ean13_png(text: &str) {
    let barcode = EAN13::new(text).unwrap();
    let png = Image::png(80);
    let encoded = barcode.encode();

    let bytes = png.generate(&encoded[..]).unwrap();

    let created_file_path = construct_file_name("ean13");

    let file = File::create(&Path::new(&created_file_path)).unwrap();

    let mut writer = BufWriter::new(file);
    writer.write(&bytes[..]).unwrap();
}

pub fn create_qrcode_png(text:&str){

    let created_file = construct_file_name("qrcode");

    qrcode_generator::to_png_to_file(text,QrCodeEcc::Medium,1024,&created_file);

}


//SUbcrates as sub folders ->>
// in the lbi.rs then you can use subcrates to keep the sections
// expose only the bits tht you want to share with the public function/ structs

