module.exports = {
  purge: ["../templates/**/*.html"],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {
      backgroundColor: ['active'],
      backgroundImage: ['active'],
      borderColor: ['active', 'hover', 'focus'],
      borderStyle: ['hover', 'focus'],
      borderWidth: ['hover', 'focus'],
      margin: ['hover', 'focus'],
      textColor: ['active']
    },
  },
  plugins: [],
}
