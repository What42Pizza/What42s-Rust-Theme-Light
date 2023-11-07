// settings

const SRC_THEME_NAME: &str = "src color-theme.json";
const OUTPUTS: &[(&str, &str)] = &[
	("white", "What42's Rust Theme (White).json"),
	("blue", "What42's Rust Theme (Blue).json"),
];

const FOLDERS_TO_EXPORT: &[&str] = &[
	"themes",
	"images",
	//"node_modules",
];
const FILES_TO_EXPORT: &[&str] = &[
	"package.json",
	"icon.png",
	"README.md",
	"CHANGELOG.md",
	"LICENSE",
];





mod data;
mod utils;

use std::{thread, time::Duration, path::PathBuf, fs};
use reqwest::StatusCode;
use serde_json::Value;
use utils::*;
use data::*;
use fs_extra::dir::{copy as copy_dir, CopyOptions as DirCopyOptions};
use fs_extra::file::{copy as copy_file, CopyOptions as FileCopyOptions};
use anyhow::*;



fn main() {
	
	let repo_path = get_repo_dir();
	let src_theme_path = repo_path.push_new(SRC_THEME_NAME);
	let themes_folder_path = repo_path.push_new("themes");
	
	static mut EXIT: bool = false;
	
	thread_join(
		
		// main thread
		|| {
			println!();
			println!("Themes builder started");
			loop {
				println!();
				println!("Choices: Export, Stop");
				let input = read_next_string().expect("Could not read console input").trim().to_lowercase();
				match &*input {
					"export" | "e" => {
						println!("Exporting extension...");
						export_extension(&repo_path);
						thread::sleep(Duration::from_millis(100));
						println!("Done");
					}
					"stop" | "s" => {
						unsafe {EXIT = true;}
						return;
					}
					_ => println!("Unkown option."),
				}
			}
		},
		
		// worker thread
		|| {
			let mut last_edit_time = src_theme_path.last_modified_time();
			build_themes(&src_theme_path, &themes_folder_path);
			loop {
				thread::sleep(Duration::from_millis(100));
				unsafe {if EXIT {return;}}
				let new_edit_time = src_theme_path.last_modified_time();
				if new_edit_time > last_edit_time {
					last_edit_time = new_edit_time;
					build_themes(&src_theme_path, &themes_folder_path);
					thread::sleep(Duration::from_millis(1000));
				}
			}
		}
		
	);
	
}





pub fn build_themes(src_theme_path: &PathBuf, themes_folder_path: &PathBuf) {
	let result = try_build_themes(src_theme_path, themes_folder_path);
	if let Err(err) = result {
		println!("Error while building themes: {err}");
	}
}

pub fn try_build_themes(src_theme_path: &PathBuf, themes_folder_path: &PathBuf) -> Result<()> {
	let src_theme = fs::read_to_string(src_theme_path)?;
	let src_theme: Theme = deser_hjson::from_str(&src_theme)?;
	
	for (style, output_name) in OUTPUTS.iter().copied() {
		let processed_theme = process_theme(&src_theme, style);
		let processed_theme_path = themes_folder_path.push_new(output_name);
		fs::write(processed_theme_path, serde_json::to_string(&processed_theme)?)?;
	}
	
	Ok(())
}



pub fn process_theme(src_theme: &Theme, style: &str) -> Theme {
	let mut output = src_theme.clone();
	
	// colors
	let mut replacements = vec!();
	for (color_key, color) in &src_theme.colors {
		let ColorValue::Diverge(styles) = color else {continue;};
		let Some(result) = styles.get(style) else {
			println!("Warning: src.colors.{color_key} does not have an option for style {style}.");
			continue;
		};
		replacements.push((color_key.clone(), result.clone()));
	}
	for (key, result) in replacements {
		output.colors.insert(key, ColorValue::Inline(result));
	}
	
	// tokenColors
	let mut foreground_replacements = vec!();
	let mut background_replacements = vec!();
	for (i, token_color) in src_theme.token_colors.iter().enumerate() {
		if let Some(ColorValue::Diverge(styles)) = token_color.settings.get("foreground") {
			let Some(result) = styles.get(style) else {
				println!("Warning: src.tokenColors.{i}.settings.foreground does not have an option for style {style}.");
				continue;
			};
			foreground_replacements.push((i, result.clone()));
		}
		if let Some(ColorValue::Diverge(styles)) = token_color.settings.get("background") {
			let Some(result) = styles.get(style) else {
				println!("Warning: src.tokenColors.{i}.settings.background does not have an option for style {style}.");
				continue;
			};
			background_replacements.push((i, result.clone()));
		}
	}
	for (i, result) in foreground_replacements {
		output.token_colors[i].settings.insert(String::from("foreground"), ColorValue::Inline(result));
	}
	for (i, result) in background_replacements {
		output.token_colors[i].settings.insert(String::from("background"), ColorValue::Inline(result));
	}
	
	output
}





pub fn export_extension(repo_path: &PathBuf) {
	let result = try_export_extension(repo_path);
	if let Err(err) = result {
		println!("Error while exporting extension: {err}");
	}
}

const EMPTY_FILE_COPY_OPTIONS: FileCopyOptions = FileCopyOptions {
	overwrite: false,
	skip_exist: false,
	buffer_size: 64000, //64kb
};
const EMPTY_DIR_COPY_OPTIONS: DirCopyOptions = DirCopyOptions {
	overwrite: false,
	skip_exist: false,
	buffer_size: 64000, // 64kb
	copy_inside: false,
	content_only: false,
	depth: 0,
};

pub fn try_export_extension(repo_path: &PathBuf) -> Result<()> {
	let output_path = repo_path.push_new("output");
	
	let repo_path_clone = repo_path.clone();
	thread::spawn(|| check_package_version(repo_path_clone));
	
	if output_path.exists() {
		fs::remove_dir_all(&output_path)?;
	}
	fs::create_dir(&output_path)?;
	
	for dir_name in FOLDERS_TO_EXPORT.iter().copied() {
		copy_dir(repo_path.push_new(dir_name), &output_path, &EMPTY_DIR_COPY_OPTIONS)?;
	}
	for file_name in FILES_TO_EXPORT.iter().copied() {
		copy_file(repo_path.push_new(file_name), output_path.push_new(file_name), &EMPTY_FILE_COPY_OPTIONS)?;
	}
	
	Ok(())
}



pub fn check_package_version(repo_path: PathBuf) {
	let result = try_check_package_version(repo_path);
	if let Err(err) = result {
		println!("Error while checking extension version: {err}");
	}
}

pub fn try_check_package_version(repo_path: PathBuf) -> Result<()> {
	
	// references:
	// https://github.com/cssho/VSMarketplaceBadges
	// https://github.com/microsoft/vsmarketplace/issues/238
	let marketplace_api_url = "https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery";
	let client = reqwest::blocking::Client::new();
	let result = client
		.post(marketplace_api_url)
		.header(reqwest::header::USER_AGENT, "What42Pizza")
		.header(reqwest::header::ACCEPT, "application/json;api-version=3.0-preview.1")
		.header(reqwest::header::CONTENT_TYPE, "application/json")
		.body(r#"{"filters":[{"criteria":[{"filterType":7,"value":"What42Pizza.what42s-rust-theme-light"}]}],"flags":16}"#)
		.send()?;
	
	if result.status() != StatusCode::OK {return Ok(());}
	let result = result.text()?;
	let result: Value = serde_json::from_str(&result)?;
	let Value::Object(result) = result else {return error("Http Post return value was not a json object");};
	
	// get version of latest upload
	let Some(results) = result.get("results") else {return error("Http Post return.results was not present");};
	let Some(results) = results.as_array() else {return error("Http Post return.results was not an array");};
	let Some(first_result) = results.get(0) else {return error("Http Post return.results was empty");};
	let Some(first_result) = first_result.as_object() else {return error("Http Post return.results[0] was not an object");};
	let Some(extensions) = first_result.get("extensions") else {return error("Http Post return.results[0] did not have entry 'extensions'");};
	let Some(extensions) = extensions.as_array() else {return error("Http Post return.results[0].etensions was not an array");};
	let Some(first_extension) = extensions.get(0) else {return error("Http Post return.results[0].extentions was empty");};
	let Some(first_extension) = first_extension.as_object() else {return error("Http Post return.results[0].extentions[0] was not an object");};
	let Some(versions) = first_extension.get("versions") else {return error("Http Post return.results[0].extentions[0] did not have entry 'versions'");};
	let Some(versions) = versions.as_array() else {return error("Http Post return.results[0].extentions[0].versions was not an array");};
	let Some(first_version) = versions.get(0) else {return error("Http Post return.results[0].extentions[0].versions was empty");};
	let Some(upload_version) = first_version.get("version") else {return error("Http Post return.results[0].extentions[0].versions[0] did not have entry 'version'");};
	let Some(upload_version) = upload_version.as_str() else {return error("Http Post return.results[0].extentions[0].versions[0].versions was not a string");};
	
	// get version of current dir
	let package = fs::read_to_string(repo_path.push_new("package.json"))?;
	let package: Value = serde_json::from_str(&package)?;
	let Some(package) = package.as_object() else {return error("Repo's package was not an object");};
	let Some(repo_version) = package.get("version") else {return error("Repo's package did not have entry 'version'");};
	let Some(repo_version) = repo_version.as_str() else {return error("Repo's package.versions was not a string");};
	
	// even more parsing
	fn parse_str_version_part(part: &str) -> u32 {
		match part.parse::<u32>() {
			Result::Ok(v) => v,
			Result::Err(err) => panic!("Error while parsing version part: {err}"),
		}
	}
	fn get_version_parts(version: &str) -> Vec<u32> {
		version
			.split('.')
			.map(parse_str_version_part)
			.collect()
	}
	let upload_parts = get_version_parts(upload_version);
	let repo_parts = get_version_parts(repo_version);
	
	// validate version number
	if repo_parts.len() != 3 {return error("Package version does not contain 3 ints");}
	if repo_parts == upload_parts {return error("Version number has not been updated");}
	
	let mut i = 0;
	'outer: loop {
		if i == 3 {break;}
		
		if upload_parts[i] > repo_parts[i] {return error("Version number is lower than the uploaded version number");}
		let version_increase = repo_parts[i] - upload_parts[i];
		match version_increase {
			0 => {i += 1; continue;}
			1 => {
				loop {
					i += 1;
					if i == 3 {break 'outer;}
					if repo_parts[i] != 0 {println!("WARNING: Version number is 2 or more versions higher than the uploaded version number"); break 'outer;}
				}
			}
			_ => {println!("WARNING: Version number is 2 or more versions higher than the uploaded version number"); break;}
		}
		
	}
	
	Ok(())
}
