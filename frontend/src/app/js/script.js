// Get id big-text
var bt = $("#big-text");
// on load function 
$(window).on("load", function () {
    // set time out function
    setTimeout(function () {
        bt.css("opacity", "1");
        bt.addClass("text-animate");
    }, 1000);
    // Set bt margin-top and font-size
});


$(document).ready(function () {

    const nav = $(".nav-container");

    if (nav.length) {
        const toggle = nav.find(".nav-toggle");

        if (toggle.length) {
            toggle.on("click", function () {
                if (nav.hasClass("is-active")) {
                    nav.removeClass("is-active");
                } else {
                    nav.addClass("is-active");
                }
            });

            nav.on("blur", function () {
                nav.removeClass("is-active");
            });
        }
    }

    // Pop up window
    const popup = $("#popup-window");

    // Make a function to open popup window with contains
    $(".button").on('click', (e) => {
        // Return case
        if (popup.hasClass("is-active"))
            return;

        var buttonId = e.currentTarget.id;
        if (buttonId == "help")
            var popupContent =
                '<div id="rules">' +
                '<h2>How to play?</h1>' +
                '<p>Welcome to "Guess the Coin Toss"! In this game, you will be prompted to choose between' +
                '"heads" or "tails". The bot will then attempt to guess your choice before it is revealed. If' +
                'the bot guesses correctly, you will lose $100 and the bot will earn $100. If the bot guesses' +
                'incorrectly, you will earn $100 and the bot will lose $100. The challenge is to outsmart the' +
                'computer by being as random as possible to earn more money. Have fun and good luck!</p>' +
                '</div>';
        else
            var popupContent = "";

        popup.addClass("is-active");
        popup.find(".popup-content").html(popupContent);

        // Stop interactivity with the background when popup is open, only allow to close popup
        nav.addClass("disable-interaction");
        $(".content").addClass("disable-interaction");
    });

    // Make a function to close popup window
    popup.find(".popup-close").on("click", () => {
        // Return case
        if (!popup.hasClass("is-active"))
            return;

        popup.removeClass("is-active");
        popup.find(".popup-content").html("");

        // Allow interactivity with the background when popup is close
        nav.removeClass("disable-interaction");
        $(".content").removeClass("disable-interaction");
    });

});

