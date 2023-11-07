const fs = require('fs-extra');
const path = require('path');
const build_themes = require('./build_themes');

const REPO_DIR = path.join(__dirname, '..');
const OUTPUT_DIR = path.join(REPO_DIR, 'output');

async function copy(path_end) {
	await fs.copy(path.join(REPO_DIR, path_end), path.join(OUTPUT_DIR, path_end));
}

module.exports = async () => {
	
	await build_themes()
	
	await fs.remove(OUTPUT_DIR);
	
	await copy('themes');
	await copy('package.json');
	await copy('README.md');
	await copy('CHANGELOG.md');
	await copy('icon.png');
	await copy('images');
	await copy('LICENSE');
	//await copy('node_modules');
	
};

if (require.main === module) {
	module.exports();
}
