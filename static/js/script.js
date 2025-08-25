function get_hello() {
    fetch("/api")
        .then((response) => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then((data) =>
            document.getElementById("greeting").innerHTML = data.message
        )
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
    // const username = 1;
    const username = "muster.dorothe@gdb.lernsax.de";
    const password =
        "a732989fad60ad40a5fda770a16e6c5853fca8c8c1cb491da1433e2954badbc2";
    const headers = new Headers();
    headers.set("Authorization", "Basic " + btoa(username + ":" + password));

    const listContainer = document.getElementById("greeting");
    const ul = document.createElement("id");

    fetch("/api/students", { headers: headers })
        .then((response) => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then((data) => {
            data.forEach((item) => {
                const li = document.createElement("li");
                li.textContent = item.first_name;
                ul.appendChild(li);
            });

            listContainer.appendChild(ul);
        })
        .catch((error) => console.error("Error:", error));
}
