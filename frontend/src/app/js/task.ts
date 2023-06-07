export function sendRefreshRequest() {
    $.ajax({
        url: '/api/refresh',
        method: 'GET',
        error: function (xhr, status, error) {
            console.error('Error sending refresh request:', error);
        }
    });
}

export function setupRefreshTimer() {
    setInterval(sendRefreshRequest, 300000); // 5 minutes = 300000 milliseconds
}