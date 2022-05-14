#![feature(stdin_forwarders)]

use std::path::PathBuf;

use walkdir::WalkDir;

enum Results {
    Empty,
    Testing(Vec<usize>),
    Success((usize, usize)),
    Errors((usize, usize)),
}

impl TryFrom<String> for Results {
    type Error = std::io::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let out = value
            .split_once(" ")
            .map(|(_, values)| values.split(","))
            .map(|x| x.flat_map(|val| val.parse::<usize>()));

        if value.starts_with("timing") {
            return Ok(Results::Testing(out.unwrap().collect()));
        } else if value.starts_with("error") {
            let value = out.unwrap().collect::<Vec<usize>>();
            return Ok(Results::Errors((
                *value.get(0).unwrap(),
                *value.get(1).unwrap()
            )));
        } else if value.starts_with("success") {
            let value = out.unwrap().collect::<Vec<usize>>();
            return Ok(Results::Success((
                *value.get(0).unwrap(),
                *value.get(1).unwrap()
            )));
        }

        return Ok(Results::Empty);
    }

}

fn process(file: PathBuf, name: String) -> Result<(), Box<dyn std::error::Error>> {
    let results: Vec<Results> = std::fs::read_to_string(file)?
        .lines()
        .flat_map(|x| x.to_string().try_into())
        .collect();

    let mut total_time: usize = 0;
    let mut times: usize = 0;
    let mut success_rate: usize = 0;
    let mut error_count: usize = 0;
    let mut error_rate: usize = 0;


    for result in results {
        match result {
            Results::Testing(vec) => {
                total_time += vec.iter().sum::<usize>();
                times += vec.len();
            },
            Results::Success(s) => {
                success_rate = s.1;
            },
            Results::Errors(e) => {
                error_count = e.0;
                error_rate = e.1;
            },
            _ => {}
        }
    }

    println!("{} average: {} rps: {}", name, total_time / times, success_rate);
    return Ok(());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::env::args().nth(1).expect("to have a data dir");
    let mut paths: Vec<PathBuf> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|file| file.ok())
        .into_iter()
        .filter(|file| !file.path().display().to_string().ends_with("mem"))
        .map(|x| x.path().to_path_buf())
        .collect();

    paths.sort();

    paths.iter().for_each(|file| {
        let display = file.display();
        process(file.to_path_buf(), display.to_string());
    });

    return Ok(());
}

