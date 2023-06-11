import { updateChart } from './plot';

const winReward = 50;
let highestEarning = 0;
export let playerBal = 1000;
let botBal = 1000;
var gramBuffer = [0, 1, 0, 1, 0]; // 5-gram buffer
var gramHistory = {}; // statistics for all 32 5-grams
var historyIndex = gramBuffer[0] * 16 + gramBuffer[1] * 8 + gramBuffer[2] * 4 + gramBuffer[3] * 2 + gramBuffer[4];
var correct = 0; // total number of correct guesses
var wrong = 0; // total number of wrong guesses
var prediction = Math.floor(Math.random() * 2); // current prediction (encoded as 0 or 1)
var lastKey = 0; // last typed key (encoded as 0 or 1)


function getChoiceAsNum(buttonId: string) {
    return buttonId === "left" ? 0 : 1;
}

$(function () {
    // initialize gram database
    for (let i = 0; i < 32; i++) { gramHistory[i] = { counter0: 0, counter1: 0 }; }
});

export function handleUserInput(buttonId: string) {
    lastKey = getChoiceAsNum(buttonId);
    if (prediction == lastKey) {
        playerBal -= winReward;
        correct++;
        botBal += winReward;
    } else {
        playerBal += winReward;
        wrong++;
        botBal -= winReward;
    }
    updateHighestEarning();
    if (checkGameOver()) {
        onGameOver();
    }
    updateUI();

    // update the 5-gram history
    gramHistory[historyIndex].counter0 += (1 - lastKey);
    gramHistory[historyIndex].counter1 += lastKey;

    // update the 5-gram buffer
    gramBuffer.push(lastKey);
    gramBuffer.shift();

    // update chart data
    var round = correct + wrong;
    updateChart(round, playerBal);

    predictNext();
}

// Take a look at the 5-gram buffer and make the next prediction
function predictNext() {
    // convert gram buffer to database index (binary to decimal)
    historyIndex = gramBuffer[0] * 16 + gramBuffer[1] * 8 + gramBuffer[2] * 4 + gramBuffer[3] * 2 + gramBuffer[4];
    // make a prediction
    if (gramHistory[historyIndex].counter1 > gramHistory[historyIndex].counter0) { prediction = 1; }
    else { prediction = 0; }
}

// Update highest earning variable whenever player earns more money
function updateHighestEarning() {
    if (playerBal <= highestEarning) { return; }
    highestEarning = playerBal;
    $("#highest-earning").html(highestEarning + "$");
}

function disableButtons() {
    // Set attribure disabled to true using jQuery
    $("#left").attr("disabled", "true");
    $("#right").attr("disabled", "true");
}

// Check if game is over
function checkGameOver() {
    return (playerBal <= 0 || botBal <= 0) ? true : false;
}

// Update UI
function updateUI() {
    $("#player-balance").html(playerBal + "$");
    $("#bot-balance").html(botBal + "$");
    $("#bot-accuracy").html(Math.round(correct / (correct + wrong + 0.0001) * 100) + "%");
}

// Game over
function onGameOver() {
    // Disable buttons and keydown event
    $(document).off('keydown');
    disableButtons();

    // Update data to server
    $.ajax({
        url: "/api/leaderboard",
        type: "POST",
        data: JSON.stringify({ "highscore": highestEarning, "accuracy": Math.round(correct / (correct + wrong + 0.0001) * 100) }),
        contentType: "application/json; charset=utf-8",
        dataType: "json",
    });
}
