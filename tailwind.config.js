export default {
  content: {
    files: ["./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  },
  theme: {
    colors: {
    },
    fontFamily: {
	},
  },
  plugins: [],
}
