import { updateChart } from './plot';

let highestEarning = 0;
export let playerBal = 1000;
let computerBal = 1000;
var gramBuffer = [0, 1, 0, 1, 0]; // 5-gram buffer
var gramHistory = {}; // statistics for all 32 5-grams
var historyIndex = gramBuffer[0] * 16 + gramBuffer[1] * 8 + gramBuffer[2] * 4 + gramBuffer[3] * 2 + gramBuffer[4];
var correct = 0; // total number of correct guesses
var wrong = 0; // total number of wrong guesses
var prediction = 0; // current prediction (encoded as 0 or 1)
var lastKey = 0; // last typed key (encoded as 0 or 1)


$(function () {
    // initialize gram database
    for (let i = 0; i < 32; i++) { gramHistory[i] = { counter0: 0, counter1: 0 }; }
});
