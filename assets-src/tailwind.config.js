module.exports = {
  purge: ["../templates/**/*.html"],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      spacing: {
        '18': '4.5rem'
      }
    },
  },
  variants: {
    extend: {
      backgroundColor: ['active'],
      backgroundImage: ['active'],
      borderColor: ['active', 'hover', 'focus'],
      borderStyle: ['hover', 'focus'],
      borderWidth: ['hover', 'focus'],
      width: ['hover', 'focus'],
      margin: ['active', 'hover', 'focus'],
      textColor: ['active']
    },
  },
  plugins: [],
}
