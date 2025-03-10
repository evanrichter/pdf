use std::env::args;

use pdf::error::PdfError;
use pdf::file::File;
use pdf::object::{FieldDictionary, FieldType, Resolve};

/// extract and print a PDF's metadata
fn main() -> Result<(), PdfError> {
    let path = args()
        .nth(1)
        .expect("Please provide a file path to the PDF you want to explore.");

    let file = File::<Vec<u8>>::open(&path).unwrap();
    if let Some(ref info) = file.trailer.info_dict {
        info.iter()
            .filter(|(_, primitive)| primitive.to_string_lossy().is_ok())
            .for_each(|(key, value)| {
                eprintln!("{:>15}: {}", key, value.to_string_lossy().unwrap());
            });
    }

    if let Some(ref forms) = file.get_root().forms {
        for field in forms.fields.iter() {
            print_field(field, &file);
        }
    }

    Ok(())
}

fn print_field(field: &FieldDictionary, resolve: &impl Resolve) {
    if field.typ == Some(FieldType::Signature) {
        println!("{:?}", field);
    }
    for &kid in field.kids.iter() {
        let child = resolve.get(kid).unwrap();
        print_field(&child, resolve);
    }
}