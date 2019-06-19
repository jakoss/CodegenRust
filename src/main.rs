extern crate byteorder;

mod input;
mod structures;

use input::*;

fn main() {
    let samples = samples_reader::get_samples("raw16").expect("Cannot read samples file");
    assert_eq!(samples.len(), 3103688);

    let test_data_vector = test_data::get_test_data("test_data").expect("Cannot read test data");
    assert_eq!(test_data_vector.len(), 6948);
}
