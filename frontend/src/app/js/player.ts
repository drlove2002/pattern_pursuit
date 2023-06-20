import { updateChart } from './plot';
import { enqueueMessage } from './utils';

const winReward = 50;
let highestEarning = 0;
export let playerBal = 1000;
let prevBal = playerBal;
let botBal = 1000;
var gramBuffer = [0, 1, 0, 1, 0]; // 5-gram buffer
var gramHistory = {}; // statistics for all 32 5-grams
var historyIndex = gramBuffer[0] * 16 + gramBuffer[1] * 8 + gramBuffer[2] * 4 + gramBuffer[3] * 2 + gramBuffer[4];
var correct = 0; // total number of correct guesses
var wrong = 0; // total number of wrong guesses
var prediction = Math.floor(Math.random() * 2); // current prediction (encoded as 0 or 1)
var lastKey = 0; // last typed key (encoded as 0 or 1)

const getStep = () => (correct + wrong);

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
        addFlotingScore(-winReward);
    } else {
        playerBal += winReward;
        wrong++;
        botBal -= winReward;
        addFlotingScore(winReward);
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
    updateChart(getStep(), playerBal);

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

function hideButtons() {
    // Set attribure disabled to true using jQuery
    $("#left").addClass("hide");
    $("#right").addClass("hide");
    $("#restart").removeClass("hide");
}

// Check if game is over
function checkGameOver() {
    return (playerBal <= 0 || botBal <= 0) ? true : false;
}

// Update UI
function updateUI() {
    const step = getStep();
    $("#player-balance").html(playerBal + "$");
    $("#bot-balance").html(botBal + "$");
    $("#bot-accuracy").html(Math.round(correct / (step + 0.0001) * 100) + "%");
    if (step && step % 20 === 0) {
        enqueueMessage(playerBal, prevBal);
        prevBal = playerBal;
    }
}

// Game over
function onGameOver() {
    if (playerBal <= 0)
        enqueueMessage(playerBal, prevBal, "Haha loser, Try again!");
    else if (botBal <= 0)
        enqueueMessage(playerBal, prevBal, "Nooo! can't believe you are more random than a random bot.");

    // Disable buttons and keydown event
    $(document).off('keydown', eventLeftRightButton)
    $(document).on('keydown', (e) => {
        if (e.key === "ArrowLeft" || e.key === "ArrowRight") {
            e.preventDefault();
        }
        if (e.key === "Enter") {
            handleRestart();
        }
    });
    hideButtons();

    const step = getStep();
    // Update data to server
    $.ajax({
        url: "/api/leaderboard",
        type: "POST",
        data: JSON.stringify({
            "highest_earning": highestEarning,
            "accuracy": Math.round(correct / (step + 0.0001) * 100),
            "steps": step,
        }),
        contentType: "application/json; charset=utf-8",
        dataType: "json",
    });
}

// Handle restart
export function handleRestart() {
    // Reset variables
    playerBal = prevBal = botBal = 1000;
    highestEarning = correct = wrong = lastKey = 0;
    prediction = Math.floor(Math.random() * 2);
    gramBuffer = [0, 1, 0, 1, 0];
    historyIndex = gramBuffer[0] * 16 + gramBuffer[1] * 8 + gramBuffer[2] * 4 + gramBuffer[3] * 2 + gramBuffer[4];

    for (let i = 0; i < 32; i++) { gramHistory[i] = { counter0: 0, counter1: 0 }; }
    updateUI()
    updateChart(0, playerBal); // Reset chart

    // Enable buttons
    $("#left").removeClass("hide");
    $("#right").removeClass("hide");
    $("#restart").addClass("hide");

    $(document).off('keydown')
    $(document).on('keydown', eventLeftRightButton);
}

export function eventLeftRightButton(e: any) {
    if (e.key == "ArrowLeft") {
        handleUserInput("left");
        doKeyPressEffect("left");
    }
    else if (e.key == "ArrowRight") {
        handleUserInput("right");
        doKeyPressEffect("right");
    }
    else if (e.key == "Enter") {
        e.preventDefault();
    }
}

function doKeyPressEffect(buttonId: string) {
    // Add key press effect to left and right button
    var btn = $("#" + buttonId);
    btn.addClass("key-pressed");
    setTimeout(function () {
        btn.removeClass("key-pressed");
    }, 100);
}

const aria = $("#choices");

// Add floating score eliment to the screen
function addFlotingScore(amount: number) {

    const animationClass = amount > 0 ? "increase" : "decrease";
    const randomX = Math.random() * aria.outerWidth() - 50;
    const randomY = Math.random() * aria.outerHeight();

    const animationElement = $('<div>', {
        text: amount > 0 ? `+${amount}` : `${amount}`,
        class: `score-animation ${animationClass}`,
        css: {
            left: randomX,
            top: randomY,
        },
    });

    aria.append(animationElement);

    setTimeout(() => {
        animationElement.remove();
    }, 1000);
}

