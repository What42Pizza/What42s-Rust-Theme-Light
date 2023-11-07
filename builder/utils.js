const hjson = require('hjson');



function object_is_diverge(object) {
	if (object.white === null) {return false;}
	if (object.blue === null) {return false;}
	if (Object.keys(object).length != 2) {return false;}
	return true;
}

function is_object(item) {
	return typeof item === 'object' && item !== null;
}

function process_possible_diverge(table, key, style) {
	let item = table[key];
	if (!is_object(item)) {return;}
	if (!object_is_diverge(item)) {return;}
	table[key] = item[style];
}



module.exports = {
	
	process_theme: async (src_theme_text, style) => {
		let output = hjson.parse(src_theme_text)
		
		// colors
		for (const key in output.colors) {
			process_possible_diverge(output.colors, key, style);
		}
		
		// tokenColors
		for (const item of output.tokenColors) {
			let item_settings = item.settings;
			if (item_settings.hasOwnProperty("foreground")) {
				process_possible_diverge(item_settings, "foreground", style);
			}
			if (item_settings.hasOwnProperty("background")) {
				process_possible_diverge(item_settings, "background", style);
			}
		}
		
		return output;
	}
	
}
