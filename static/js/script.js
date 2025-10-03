// set greeting
if (getCookie("username") !== "") {
    document.getElementById("greeting").innerHTML = "Hallo <b>" + getCookie("username") + "</b> 🚀";
}

// set cookies on login
document.getElementById("loginForm").addEventListener("submit", function(event) {
    event.preventDefault();

    const username = document.getElementById("username").value;
    const token = document.getElementById("token").value;

    const headers = new Headers();
    headers.set("Authorization", "Basic " + btoa(username + ":" + token));

    const students = document.getElementById("students");

    fetch("/api/authed", { headers: headers })
        .then((response) => {
            if (!response.ok) {
                alert("Konnte nicht authentifizieren, bitte überprüfe deine Anmeldedaten");
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then(() => {
            document.cookie = "username=" + username + ";";
            document.cookie = "token=" + token + ";";

            location.reload();
        })
        .catch((error) => console.error("Error:", error));
});

// helper function to get a cookie
function getCookie(cookieName) {
    const cookies = document.cookie.split('; ');
    for (const cookie of cookies) {
        const [name, value] = cookie.split('=');
        if (name === cookieName) {
            return decodeURIComponent(value);
        }
    }
    return null;
}

// logout user session
function logout() {
    document.cookie = "username=;";
    document.cookie = "token=;";

    document.getElementById("greeting").innerHTML = "";

    location.reload();
}

// API health check GET
function get_hello() {
    fetch("/api")
        .then((response) => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then((data) => document.getElementById("greeting").innerHTML = data)
        .catch((error) => console.error("Error:", error));
}

// API health check POST
async function post_hello() {
    const data = { message: "Saljutations!" };
    console.log(JSON.stringify(data));
    try {
        const response = await fetch("/api", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const responseData = await response.json();
        console.log(responseData);
    } catch (error) {
        console.error("Error posting JSON:", error);
    }
}

// get list of students
function get_students() {
    // const username = 2;
    // const username = "musterfrau.maxi@gdb.lernsax.de";
    // const token =
    //     "27a1ee4411f01912693e7e37295b1cf0b4eadf9f3bcdfbdefa125342431ddee2";

    const username = getCookie("username");
    const token = getCookie("token");

    const headers = new Headers();
    headers.set("Authorization", "Basic " + btoa(username + ":" + token));

    const students = document.getElementById("students");

    fetch("/api/students", { headers: headers })
        .then((response) => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then((data) => {
            // fill student list
            data.forEach((item) => {
                const option = document.createElement("option");
                option.textContent = item.first_name;
                option.setAttribute("value", item.id);
                if (item.email === username) {
                    option.selected = true;
                }
                students.appendChild(option);
            });
        })
        .catch((error) => console.error("Error:", error));
}
