// settings

const SRC_THEME_NAME: &str = "src color-theme.json";
const OUTPUTS: &[(&str, &str)] = &[
	("white", "What42's Rust Theme (White).json"),
	("blue", "What42's Rust Theme (Blue).json"),
];

const FOLDERS_TO_EXPORT: &[&str] = &[
	"themes",
	"images",
	"node_modules",
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
		if let Some(foreground) = token_color.settings.get("foreground") {
			if let ColorValue::Diverge(styles) = foreground {
				let Some(result) = styles.get(style) else {
					println!("Warning: src.tokenColors.{i}.settings.foreground does not have an option for style {style}.");
					continue;
				};
				foreground_replacements.push((i, result.clone()));
			}
		}
		if let Some(background) = token_color.settings.get("background") {
			if let ColorValue::Diverge(styles) = background {
				let Some(result) = styles.get(style) else {
					println!("Warning: src.tokenColors.{i}.settings.background does not have an option for style {style}.");
					continue;
				};
				background_replacements.push((i, result.clone()));
			}
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
