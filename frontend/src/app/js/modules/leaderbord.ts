// Create leaderboard type
export type Leaderboard = {
    rank: number,
    name: string,
    pfp: string,
    rating: number,
    highscore: number,
    accuracy: number
    steps: number
}

function getLeaderbordData(): Promise<Leaderboard[]> {
    return new Promise((resolve, reject) => {
        $.ajax({
            url: "/api/leaderboard",
            type: "GET",
            success: function (data) {
                // Get json data
                data = data.data;
                // Create leaderboard array
                let leaderboard: Leaderboard[] = [];
                // Loop through data
                for (let i = 0; i < data.length; i++) {
                    // Push data to leaderboard array
                    leaderboard.push({
                        rank: data[i].rank,
                        name: data[i].name,
                        pfp: data[i].pfp,
                        rating: data[i].rating,
                        highscore: data[i].highscore,
                        accuracy: data[i].accuracy,
                        steps: data[i].steps
                    });
                }

                // Resolve leaderboard array
                resolve(leaderboard);
            },
            error: function (error) {
                reject(error);
            }
        });
    });
}

function createLeaderboardTableHeader(): string {
    return `
    <tr>
        <th></th>
        <th></th>
        <th></th>
        <th>Name</th>
        <th>Rating</th>
        <th>Highest Earning</th>
        <th>Bot Accuracy</th>
        <th>Steps Survived</th>
    </tr>
    `;
}

// Create leaderboard table string and return it
function createLeaderboardTable(leaderboard: Leaderboard[]): string {
    // Create leaderboard table string
    let leaderboardTable = "";
    // Loop through leaderboard array
    for (let i = 0; i < leaderboard.length; i++) {
        // Add leaderboard row to leaderboard table string
        leaderboardTable += `
        <tr class="row">
            <td class="lb-rank">${leaderboard[i].rank}</td>
            <td><img src="${leaderboard[i].pfp}" alt="Profile picture"></td>
            <td>${leaderboard[i].name}</td>
            <td>${leaderboard[i].rating}</td>
            <td>${leaderboard[i].highscore}$</td>
            <td>${leaderboard[i].accuracy}%</td>
            <td>${leaderboard[i].steps}</td>
        </tr>
        `;
    }

    // Return leaderboard table string
    return leaderboardTable;
}

// Asynchronously create leaderboard table
// Start loading animation
// Get leaderboard data 
// Create leaderboard table string
// Stop loading animation
// Add leaderboard table string to leaderboard table
export function genLeaderboard() {
    // Get leaderboard table
    const leaderboardTable = $(".leaderboard");
    // Start loading animation
    leaderboardTable.html(`
    <tr>
        <td colspan="5" class="loading">
            <img src="https://cdn-icons-gif.flaticon.com/8722/8722704.gif" alt="Loading" class="loading__icon">
        </td>
    </tr>
    `);

    // Get leaderboard data
    getLeaderbordData().then((leaderboard) => {
        // Create leaderboard table string
        let leaderboardContent = createLeaderboardTableHeader() + createLeaderboardTable(leaderboard);

        // Stop loading animation
        leaderboardTable.html(leaderboardContent);
    }).catch((error) => {
        console.log(error);
        // Stop loading animation
        leaderboardTable.html(`
        <tr>
            <td colspan="5" class="loading">
                <div class="loading__error">Error loading leaderboard</div>
            </td>
        </tr>
        `);
    });
}