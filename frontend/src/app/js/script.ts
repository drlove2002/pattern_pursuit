import Cookies from 'js-cookie';
import { setupRefreshTimer } from './task';
import { plotInit } from './plot';
import { handleUserInput, handleRestart, eventLeftRightButton } from './player';
import { createLeaderboard } from './leaderbord';

function setProfile() {
    // Get profile data from /api/users/me
    $.ajax({
        url: "/api/users/me",
        type: "GET",
        headers: {}
    }).done(function (data) {
        data = data.data;
        // Set profile data
        $("#profile-picture").attr("src", data.picture);
        $("#profile-name").html(data.name);
        $("#profile-email").html(data.email);
    }).fail(function (e) {
        // If fail, remove cookie
        Cookies.remove("login");
        location.replace("/api/login");
    })
}

const popup = $("#popup-window");
const nav = $(".nav-container");

function openPopup(popupContent: string) {
    popup.addClass("is-active");
    popup.find(".popup-content").html(popupContent);

    // Stop interactivity with the background when popup is open, only allow to close popup
    nav.addClass("disable-interaction");
    $(".content").addClass("disable-interaction");
}

$(function () {
    // Add keyboard event listener for left and right button
    $(document).on('keydown', eventLeftRightButton);

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

    // Make a function to open popup window with contains
    $("button").on('click', (e) => {
        // Return case
        if (popup.hasClass("is-active"))
            return;

        var buttonId = e.currentTarget.id;
        if (buttonId == "help")
            var popupContent = `
            <div id="rules">
            <h2>How to play?</h2>
            <p>You'll be presented with a choice between "<strong>left</strong>" or "<strong>right</strong>". 
                My task is to guess your selection before it's revealed. 
                If I guess right, I take some money from you. 
                But if I guess wrong, you'll get money from me. 
                The key is to outsmart me by being as <em>random</em> as possible, which will help you earn more money.
            </p>
            </div>`;
        else if (buttonId == "left" || buttonId == "right") {
            handleUserInput(buttonId);
            return;
        }
        else if (buttonId == "restart") {
            handleRestart();
            return;
        }
        else if (buttonId == "lb-btn") {
            // Set popup content to empty leaderboard table with #leaderboard-table
            var popupContent = "<table class='leaderboard'></table>";
            openPopup(popupContent);
            createLeaderboard();
            return;
        }
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
    if (Cookies.get("login") && Cookies.get("login") == "true") {
        setProfile();
        setupRefreshTimer();
    }
    else
        location.replace("/");

    plotInit();
});
