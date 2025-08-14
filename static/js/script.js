function get_hello() {
     fetch('/api/hello')
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json();
        })
        .then(data => document.getElementById("greeting").innerHTML = data.message)
        .catch(error => console.error('Error:', error));
}

async function post_hello() {
    const data = {message: "Saljutations!"};
    console.log(JSON.stringify(data));
    try {
        const response = await fetch("/api/greet", {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data),
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const responseData = await response.json();
        console.log(responseData);
    } catch (error) {
        console.error('Error posting JSON:', error);
    }
}
