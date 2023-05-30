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


function setProfile() {
    // Get profile data from /api/users/me
    $.ajax({
        url: "/api/users/me",
        type: "GET",
        headers: {}
    }).done(function (data) {
        data = data.data;
        // Set profile data
        $("#profile-picture").attr("src", data.photo);
        $("#profile-name").html(data.name);
        $("#profile-email").html(data.email);

        // Make sign-in button to sign-out button
        const login = $("#login");
        login.attr("id", "logout");
        login.find(".button__text").html("Sign out");
    });
}

function unsetProfile() {  // TODO: Fix this
    $.ajax({
        url: "/api/oauth/logout",
        type: "GET",
        headers: {}
    }).done(function (data) {
        data = data.data;
        // Set profile data
        $("#profile-picture").attr("src", data.photo);
        $("#profile-name").html(data.name);
        $("#profile-email").html(data.email);

        // Make sign-in button to sign-out button
        const login = $("#login");
        login.attr("id", "logout");
        login.find(".button__text").html("Sign out");
    });
}

const popup = $("#popup-window");
const nav = $(".nav-container");

function openPopup(popupContent) {
    popup.addClass("is-active");
    popup.find(".popup-content").html(popupContent);

    // Stop interactivity with the background when popup is open, only allow to close popup
    nav.addClass("disable-interaction");
    $(".content").addClass("disable-interaction");
}


$(document).ready(function () {


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

    // If user is not logged in, show login button in popup
    if (!Cookies.get("login") || Cookies.get("login") == "false") {
        console.log("Not logged in");
        var popupContent =
            '<h2>Before continuing you must login.</h2>' +
            '<div class="google-btn">' +
            '<a href="/api/login">' +
            '<div class="google-icon-wrapper">' +
            '<img class="google-icon" src="https://upload.wikimedia.org/wikipedia/commons/5/53/Google_%22G%22_Logo.svg" />' +
            '</div>' +
            '<p class="btn-text"><b>Sign in with google</b></p>' +
            '</a>' +
            '</div>';
        popup.find(".popup-close").hide();
        openPopup(popupContent);
    }


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
        else if (buttonId == "login")
            var popupContent = '<div id="g_id_signin"></div>';
        else
            var popupContent = "<h1>Coming soon...</h1>";

        openPopup(popupContent);

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

    //  Update the profile picture and name when user logged in
    // Chech if we have token in cache
    if (Cookies.get("login")) {
        setProfile();
    }

});

