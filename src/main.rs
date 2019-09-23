extern crate gtk;


mod gui{
    pub mod gtk3;
}



//mod lib;
///Have the generation funcitons within the lib
/// Then  make the actix crate a parent of the barcode engine
/// This will then spit out the generated barcodes
///
mod generator;
use barcodes::create_qrcode_png;

//mod generator{





fn main(){

//    create_qrcode_png("The nail understands")
 //   let date = lib::get_fmt_date();

gui::gtk3::launch();


}
