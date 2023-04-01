import './style.css'
import { setupCounter } from './counter.js'
import { Chart as wasmChart } from 'wasm-build';

// document.querySelector('#app').innerHTML = `
//   <div>
//     <h1>Vite Test</h1>
//     <div class="card">
//       <button id="counter" type="button"></button>
//     </div>
//     <p class="read-the-docs">
//       Click on the Vite logo to learn more
//     </p>
//   </div>
// `

class Chart {};
Chart = wasmChart;
const canvas = document.getElementById("canvas");
const coord = document.getElementById("coord");
let chart = null;
console.log(wasmChart);
chart = Chart.draw("canvas", [], [])

