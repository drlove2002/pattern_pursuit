declare const Plotly: typeof import('plotly.js');
import { playerBal } from './player';

const plotDiv = $("#plot")[0]

// Create trace object
var trace = {
    x: [0],
    y: [playerBal],
    mode: 'lines',
    name: 'Player Balance'
};

// Create layout object
var layout = {
    title: 'Player Balance Over Time',
    xaxis: {
        title: 'Round'
    },
    yaxis: {
        title: 'Balance ($)'
    }
};

// Add trace to plot data
var data = [trace];


// Function to update chart data
export function updateChart(round: number, playerBalance: number) {
    Plotly.extendTraces(plotDiv, { x: [[round]], y: [[playerBalance]] }, [0]);
}


// Initialize plot
export function plotInit() {
    Plotly.newPlot(plotDiv, data, layout, { staticPlot: true });
    $('.main-svg[style]').css('background', 'rgb(255 255 255 / 80%)');
    $('.main-svg[style]').css('border-radius', '10px');
    $('.svg-container').removeAttr('style');
}