use std::fs::File;

#[derive(serde::Deserialize)]
struct CsvData {
    id: u32,
    words: String,
    iamge_url: String,
    _twitter_url: String,
    _tags: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let save_path = "target/images/";
    let csv_path = "data/aikatsup20230422.csv";
    let file = File::open(csv_path)?;
    let mut reader = csv::ReaderBuilder::new().flexible(true).from_reader(file);

    for record in reader.records() {
        let csv_data: CsvData = record?.deserialize(None)?;
        let id = csv_data.id;
        let url = csv_data.iamge_url;
        let image_bytes = reqwest::get(&url).await?.bytes().await?;
        let mut saving_file = File::create(format!("{save_path}{id}.jpg"))?;
        std::io::copy(&mut image_bytes.as_ref(), &mut saving_file)?;

        println!("{id}: {} from {url}", csv_data.words);
        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    }

    Ok(())
}
