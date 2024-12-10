use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Context;

pub struct FileData {
    pub reports: Vec<Vec<i32>>,
}

impl FileData {
    pub fn new(contents: Vec<Vec<i32>>) -> Self {
        Self { reports: contents }
    }
}

pub fn parse_file_data(buffer: BufReader<File>) -> anyhow::Result<FileData> {
    let mut fileData: FileData = FileData::new(Vec::new());

    for line in buffer.lines() {
        let text = &line.context("Could not get the line text!")?;
        let report: Vec<i32> = text
            .split(" ")
            .filter_map(|item| item.parse::<i32>().ok())
            .collect();

        fileData.reports.push(report);
    }

    Ok(fileData)
}
