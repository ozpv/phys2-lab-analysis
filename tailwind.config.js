export default {
  content: {
    files: ["./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  },
  theme: {
    colors: {
		base: '#eff1f5',
		mantle: '#e6e9ef',
		crust: '#dce0e8',
		blue: '#1e66f5',
	 	sky: '#04a5e5',
		surface: {
			default: '#ccd0da',
			100: '#bcc0cc',
			200: '#acb0be',
		},
		text: '#4c4f69',
		subtext: {
			default: '#5c5f77',
			100: '#5c5f77',
		},
    },
    fontFamily: {
	},
  },
  plugins: [],
}
