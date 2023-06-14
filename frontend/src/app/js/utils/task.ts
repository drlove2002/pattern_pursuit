import Cookies from 'js-cookie';

function sendRefreshRequest() {
    $.ajax({
        url: '/api/refresh',
        method: 'GET',
        error: function (xhr, status, error) {
            console.error('Error sending refresh request:', error);
        }
    });
}

function initRefreshTimer() {
    setInterval(sendRefreshRequest, 300000); // 5 minutes = 300000 milliseconds
}

function setProfile() {
    return new Promise<void>((resolve, reject) => {
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
            resolve(); // Resolve the promise on success
        }).fail(function (e) {
            // If fail, remove cookie
            Cookies.remove("login");
            location.replace("/api/login");
            reject(e); // Reject the promise on failure
        });
    });
}

export { setProfile, initRefreshTimer };