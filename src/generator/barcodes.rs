extern crate barcoders;

use barcoders::sym::codabar::*;
use barcoders::sym::code11::*;
use barcoders::sym::code39::*;
use barcoders::sym::code93::*;
use barcoders::sym::code128::*;
use barcoders::sym::ean8::*;
use barcoders::sym::ean13::*;

mod generator{

         fn getFmtDate()->String{
            let utc: DateTime<Utc> = Utc::now();
            let formatted_date = utc.format("%d-%m-%y %H:%M:%S").to_string();
            formatted_date;
        }

    pub mod QR{

    }

    pub mod codabar{}

    pub mod code11{}

     mod code39{
        pub fn create_code39PNG(text:String){

            //creates barcode
            let barcode =Code39::new(text).unwrap();
            let png  = Image::png(80);
            let encoded = barcode.encode();

            // generators return a Result<Vec<u8>, barcoders::error::Error) of encoded bytes
            let bytes = png.generate(&encoded[..]).unwrap();

            let date = getFmtDate();

            let file = File::create(&Path::new("src/resources/generated/code39/code39"+ date+".png")).unwrap();

            let mut writer = BufWriter::new(file);
            writer.write(&bytes[..]).unwrap();

        }

    }

    pub mod code93{}

    pub mod code128{}

    pub mod ean8{}

    pub mod ean13{}

}

//fn main(){
//    code39::create_code39PNG("Hammer head");
//}