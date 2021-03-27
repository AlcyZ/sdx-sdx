module.exports = {
	mount: {
		"ui": "/"
	},
	
	optimize: {
		minify: true,
		bundle: true
	},
	
	plugins: [
		'@snowpack/plugin-typescript',
		'@snowpack/plugin-vue',
		'@snowpack/plugin-sass',
	],
};
