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
});

