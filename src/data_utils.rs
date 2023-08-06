use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Result, Write},
};

pub fn combine_data() -> Result<()> {
    let mut data_file_writer = BufWriter::new(File::create("data/data.csv")?);
    let header = "Ticker,Date,Open,High,Low,Close,Adj Close,Volume";
    data_file_writer.write_all(header.as_bytes())?;
    data_file_writer.write(b"\n")?;

    for year in 2008..2024 {
        let file_name = format!("data/{}_Global_Markets_Data.csv", year);
        let reader = BufReader::new(File::open(file_name)?);
        let mut line_iter = reader.lines().skip(1);
        while let Some(Ok(line)) = line_iter.next() {
            data_file_writer.write_all(line.as_bytes())?;
            data_file_writer.write(b"\n")?;
        }
        data_file_writer.flush()?;
    }
    Ok(())
}
