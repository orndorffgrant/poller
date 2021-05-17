import resolve from '@rollup/plugin-node-resolve';

export default {
  input: 'src/main.js',
  output: {
    file: '../assets/main.js',
    format: 'iife'
  },
  plugins: [ resolve() ]
};