const fs = require('fs');
const path = require('path');
const utils = require('./utils');

const THEMES_DIR = path.join(__dirname, '..', '..', 'themes');

if (!fs.existsSync(THEMES_DIR)) {
	fs.mkdirSync(THEMES_DIR);
}

module.exports = async () => {
	let base_theme_text = await fs.promises.readFile(path.join(__dirname, '..', "base color-theme.json"), 'utf-8');
	
	let white = await utils.process_theme(base_theme_text, "white");
	let blue = await utils.process_theme(base_theme_text, "blue");
	
	return Promise.all([
		fs.promises.writeFile(
			path.join(THEMES_DIR, "What42's Rust Theme (White).json"),
			JSON.stringify(white)
		),
		fs.promises.writeFile(
			path.join(THEMES_DIR, "What42's Rust Theme (Blue).json"),
			JSON.stringify(blue)
		),
	]);
};

if (require.main === module) {
	module.exports();
}
