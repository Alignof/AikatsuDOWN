use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let path = "data/aikatsup20230422.csv";
    let file = File::open(path)?;
    let mut reader = csv::ReaderBuilder::new().flexible(true).from_reader(file);

    for record in reader.records() {
        println!("{:?}", record?);
    }

    Ok(())
}
