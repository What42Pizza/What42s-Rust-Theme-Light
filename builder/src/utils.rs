use std::{path::{PathBuf, Path}, thread, io, time::SystemTime};
use anyhow::*;



pub trait PathBufTraits {
	fn contains_file(&self, dir_name: &str) -> bool;
}

impl PathBufTraits for PathBuf {
	fn contains_file(&self, dir_name: &str) -> bool {
		for path in self.read_dir().expect("Could not read dir entries") {
			if path.expect("Could not process dir entry").file_name() == dir_name {return true;}
		}
		false
	}
}



pub trait PathTraits {
	fn last_modified_time(&self) -> SystemTime;
}

impl PathTraits for Path {
	fn last_modified_time(&self) -> SystemTime {
		let meta_data = self.metadata().unwrap_or_else(|err| {
			panic!("Could not retrieve metadata for file '{:?}': {}", self, err);
		});
		meta_data.modified().unwrap_or_else(|err| {
			panic!("Could not retrieve last modified time for file '{:?}': {}", self, err);
		})
	}
}



pub fn get_repo_dir() -> PathBuf {
	let mut output = get_program_dir();
	while !output.contains_file("package.json") {
		let popped = output.pop();
		if !popped {panic!("Could not find 'package.json' in any parent directories");}
	}
	output
}



pub fn get_program_dir() -> PathBuf {
	let mut output = std::env::current_exe()
		.expect("Could not retrieve the path for the current exe.");
	output.pop();
	output
}



pub fn thread_join(fn_1: impl Fn() + Send, fn_2: impl Fn() + Send) {
	thread::scope(|s| {
		
		let thread_one = s.spawn(fn_1);
		let thread_two = s.spawn(fn_2);
		
		thread_one.join().unwrap();
		thread_two.join().unwrap();
	});
}



pub fn read_next_string() -> Result<String> {
	let mut input_string = String::new();
	io::stdin().read_line(&mut input_string)?;
	Ok(input_string)
}



pub fn error<T>(msg: &'static str) -> Result<T> {
	Err(Error::msg(msg))
}
