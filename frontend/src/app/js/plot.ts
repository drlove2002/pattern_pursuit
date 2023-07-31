declare const Plotly: typeof import('plotly.js');
import { playerBal } from './player';

const plotDiv = $("#plot")[0]

var round = -1;
function createTrace() {
    // Create trace object
    return {
        x: [0],
        y: [playerBal],
        mode: 'lines',
        name: `Round-${round + 1}`,
    };
}

// Create layout object
var layout = {
    title: 'Balance Over Time',
    xaxis: {
        title: 'Steps'
    },
    yaxis: {
        title: 'Your Balance ($)'
    }
};

// Function to restart the chart with a new episode
function restartChart() {
    round++;
    var trace = createTrace();
    Plotly.addTraces(plotDiv, trace);

    // after every 10 rounds, remove all previous traces
    if (round % 10 === 0) {
        Plotly.deleteTraces(plotDiv, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

// Function to update chart data
export function updateChart(step: number, playerBalance: number) {
    if (step === 0)
        restartChart();
    else
        Plotly.extendTraces(plotDiv, { x: [[step]], y: [[playerBalance]] }, [round % 10]);
}


// Initialize plot
export function plotInit() {
    round++;
    Plotly.newPlot(plotDiv, [createTrace()], layout, { staticPlot: true });
    $('.main-svg[style]').css('background', 'rgb(255 255 255 / 80%)');
    $('.main-svg[style]').css('border-radius', '10px');
    $('.svg-container').removeAttr('style');
}