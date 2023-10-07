mod convert;
mod extract;

use std::path::Path;

fn main() {
    let file_path = Path::new("local/Gintama, v01 [2004] [Viz] [senfgurke2].cbz");
    let operation_folder = Path::new("local/extracted");
    let destination_folder = Path::new("local/converted");

    extract::extract_into_folder(file_path, operation_folder)
        .expect("unable to extract into folder");

    convert::convert_to_pdf(operation_folder, destination_folder)
        .expect("unable to convert to pdf");
}
