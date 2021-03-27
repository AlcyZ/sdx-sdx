module.exports = {
	mount: {
		"ui": "/"
	},
	
	optimize: {
		minify: false,
		bundle: false,
	},
	
	plugins: [
		'@snowpack/plugin-typescript',
		'@snowpack/plugin-vue',
		'@snowpack/plugin-sass',
	],
};
