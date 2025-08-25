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

function get_students() {
    // const username = 2;
    const username = "musterfrau.maxi@gdb.lernsax.de";
    const password =
        "27a1ee4411f01912693e7e37295b1cf0b4eadf9f3bcdfbdefa125342431ddee2";
    const headers = new Headers();
    headers.set("Authorization", "Basic " + btoa(username + ":" + password));

    const listContainer = document.getElementById("greeting");
    const select = document.createElement("select");

    fetch("/api/students", { headers: headers })
        .then((response) => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then((data) => {
            data.forEach((item) => {
                const option = document.createElement("option");
                option.textContent = item.first_name;
                option.setAttribute("value", item.id);
                if (item.email === username) {
                    option.selected = true;
                }
                select.appendChild(option);
            });

            listContainer.appendChild(select);
        })
        .catch((error) => console.error("Error:", error));
}
