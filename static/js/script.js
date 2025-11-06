// ### TESTING USER
// username: 2
// username: "musterfrau.maxi@gdb.lernsax.de"
// token: "27a1ee4411f01912693e7e37295b1cf0b4eadf9f3bcdfbdefa125342431ddee2"
// authstr: Basic bXVzdGVyZnJhdS5tYXhpQGdkYi5sZXJuc2F4LmRlOjI3YTFlZTQ0MTFmMDE5MTI2OTNlN2UzNzI5NWIxY2YwYjRlYWRmOWYzYmNkZmJkZWZhMTI1MzQyNDMxZGRlZTI=

// WARNING! This is terrible JS code!
// I had no JS experience and was under time pressure.
// PRs are very welcome!
// https://github.com/Rabenstern/abizeitungsumfrage

// set greeting
if (getCookie("username") !== "" && getCookie("username") !== null) {
    const greeting = document.getElementById("greeting");

    if (greeting !== null) {
        greeting.innerHTML = "Hallo <b>" + getCookie("username") + "</b> 🚀";
    }
}

// login user
if (document.getElementById("loginForm") != null) {
    // set cookies on login
    document.getElementById("loginForm").addEventListener("submit", function(event) {
        event.preventDefault();

        const username = document.getElementById("username").value;
        const token = document.getElementById("token").value;

        const headers = new Headers();
        headers.set("Authorization", "Basic " + btoa(username + ":" + token));

        fetch("/api/authed", { headers: headers })
            .then((response) => {
                if (!response.ok) {
                    alert("Konnte nicht authentifizieren, bitte überprüfe deine Anmeldedaten ⚠️");
                    throw new Error("Network response was not ok: ", response.status);
                }

                document.cookie = "username=" + username + ";";
                document.cookie = "token=" + token + ";";

                location.href = "questions.html";

                return response.json();
            })
            .catch((error) => console.error("Error:", error));
    });
}

if (document.getElementById("logoutBtn") !== null) {
    document.getElementById("logoutBtn").addEventListener("click", logout);
}

if (document.getElementById("saveBtn") !== null) {
    document.getElementById("saveBtn").addEventListener("click", save_questions);
}

if (document.getElementById("questions") !== null) {
    load_questions();
}

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

    location.href = "index.html";
}

function build_authstr() {
    const username = getCookie("username");
    const token = getCookie("token");
    return "Basic " + btoa(username + ":" + token);
}

// get list of students
async function get_students() {
    const authstr = build_authstr();

    const headers = new Headers();
    headers.set("Authorization", authstr);

    const res = await fetch("/api/students", { headers: headers });
    if (!res.ok) {
        throw new Error("Network response was not ok: ", res.status);
    }
    return res.json();
}

// get student via their email adress (username)
async function get_students_with_email(email) {
    const authstr = build_authstr();

    const headers = new Headers();
    headers.set("Authorization", authstr);
    const url = "/api/students?email=" + email;

    const res = await fetch(url, { headers: headers });
    if (!res.ok) {
        throw new Error("Network response was not ok: ", res.status);
    }
    return res.json();
}

// get list of teachers
async function get_teachers() {
    const authstr = build_authstr();

    const headers = new Headers();
    headers.set("Authorization", authstr);

    const res = await fetch("/api/teachers", { headers: headers });
    if (!res.ok) {
        throw new Error("Network response was not ok: ", res.status);
    }
    return res.json();
}

// get list of questions
async function get_questions() {
    const authstr = build_authstr();

    const headers = new Headers();
    headers.set("Authorization", authstr);

    const res = await fetch("/api/questions", { headers: headers });
    if (!res.ok) {
        throw new Error("Network response was not ok: ", res.status);
    }

    return res.json();
}

// get list of answers
async function get_answers(sid) {
    const authstr = build_authstr();

    const headers = new Headers();
    headers.set("Authorization", authstr);

    const url = "/api/answers?sid=" + sid;

    const res = await fetch(url, { headers: headers });
    if (!res.ok) {
        throw new Error("Network response was not ok: ", res.status);
    }

    return res.json();
}

// post student's answers to server
async function post_answer(a) {
    const authstr = build_authstr();

    const headers = new Headers();
    headers.set("Authorization", authstr);
    headers.set("Content-type", "application/json");

    const res = await fetch("/api/answer", {
        method: "POST",
        headers: headers,
        body: JSON.stringify(a),
    });
    if (!res.ok) {
        throw new Error("Network response was not ok: ", res.status);
    }
    return res.json();
}

// load questions into page
async function load_questions() {
    try {
        const students = await get_students();
        if (!Array.isArray(students)) {
            console.error("get_students() did not return an array");
        }
        students.sort(function(a, b) {
            if (a.last_name < b.last_name) {
                return -1;
            }
            if (a.last_name > b.last_name) {
                return 1;
            }
            return 0;
        });

        const teachers = await get_teachers();
        if (!Array.isArray(teachers)) {
            console.error("get_teachers() did not return an array");
        }
        teachers.sort(function(a, b) {
            if (a.last_name < b.last_name) {
                return -1;
            }
            if (a.last_name > b.last_name) {
                return 1;
            }
            return 0;
        });

        const data = await get_questions();
        if (!Array.isArray(data)) {
            console.error("get_questions() did not return an array");
        }
        data.sort(function(a, b) {
            if (a.id < b.id) {
                return -1;
            }
            if (a.id > b.id) {
                return 1;
            }
            return 0;
        });

        // get current student
        const username = getCookie("username");
        const s = await get_students_with_email(username);

        // student answers
        const answers = await get_answers(s[0].id);
        if (!Array.isArray(answers)) {
            console.error("get_answers() did not return an array");
        }

        const questions = document.getElementById("questions");

        // build questions
        data.forEach(async (item) => {
            const question = document.createElement("div");
            question.className = "question";
            question.dataset.qid = item.id;

            const answer = answers.filter((a) => a.qid === item.id)[0];

            const q = document.createElement("h2");
            q.textContent = item.q;
            question.appendChild(q);

            const optDiv = document.createElement("div");
            optDiv.className = "optDiv";

            // load option selectors
            if (item.opt1) {
                const opt = document.createElement("select");

                let sel;
                if (answer && answer.opt1) {
                    sel = answer.opt1;
                } else {
                    const preselect = document.createElement("option");
                    preselect.selected = true;
                    preselect.disabled = true;
                    preselect.innerHTML = "Bitte auswählen";
                    opt.appendChild(preselect);
                }

                opt.dataset.opt = 1;

                if (item.opt1 === "Student") {
                    students.forEach((s) => {
                        const option = document.createElement("option");
                        option.value = s.id;
                        option.textContent = `${s.last_name}, ${s.first_name}`;

                        if (typeof sel !== undefined && sel === s.id) {
                            option.selected = true;
                        }
                        opt.appendChild(option);
                    });
                } else if (item.opt1 === "Teacher") {
                    teachers.forEach((s) => {
                        const option = document.createElement("option");
                        option.value = s.id;
                        option.textContent = `${s.last_name}, ${s.first_name}`;

                        if (sel !== undefined && sel === s.id) {
                            option.selected = true;
                        }
                        opt.appendChild(option);
                    });
                }

                optDiv.appendChild(opt);
            }

            if (item.opt2) {
                const opt = document.createElement("select");

                let sel;
                if (answer && answer.opt2) {
                    sel = answer.opt2;
                } else {
                    const preselect = document.createElement("option");
                    preselect.selected = true;
                    preselect.disabled = true;
                    preselect.innerHTML = "Bitte auswählen";
                    opt.appendChild(preselect);
                }

                opt.dataset.opt = 2;

                if (item.opt2 === "Student") {
                    students.forEach((s) => {
                        const option = document.createElement("option");
                        option.value = s.id;
                        option.textContent = `${s.last_name}, ${s.first_name}`;

                        if (sel !== undefined && sel === s.id) {
                            option.selected = true;
                        }
                        opt.appendChild(option);
                    });
                } else if (item.opt2 === "Teacher") {
                    teachers.forEach((s) => {
                        const option = document.createElement("option");
                        option.value = s.id;
                        option.textContent = `${s.last_name}, ${s.first_name}`;

                        if (sel !== undefined && sel === s.id) {
                            option.selected = true;
                        }
                        opt.appendChild(option);
                    });
                }

                optDiv.appendChild(opt);
            }

            if (item.opt3) {
                const opt = document.createElement("select");

                let sel;
                if (answer && answer.opt3) {
                    sel = answer.opt3;
                } else {
                    const preselect = document.createElement("option");
                    preselect.selected = true;
                    preselect.disabled = true;
                    preselect.innerHTML = "Bitte auswählen";
                    opt.appendChild(preselect);
                }

                const preselect = document.createElement("option");
                preselect.selected = true;
                preselect.disabled = true;
                preselect.innerHTML = "Bitte auswählen";
                opt.appendChild(preselect);
                opt.dataset.opt = 3;

                if (item.opt3 === "Student") {
                    students.forEach((s) => {
                        const option = document.createElement("option");
                        option.value = s.id;
                        option.textContent = `${s.last_name}, ${s.first_name}`;

                        if (sel !== undefined && sel === s.id) {
                            option.selected = true;
                        }
                        opt.appendChild(option);
                    });
                } else if (item.opt3 === "Teacher") {
                    teachers.forEach((s) => {
                        const option = document.createElement("option");
                        option.value = s.id;
                        option.textContent = `${s.last_name}, ${s.first_name}`;

                        if (sel !== undefined && sel === s.id) {
                            option.selected = true;
                        }
                        opt.appendChild(option);
                    });
                }

                optDiv.appendChild(opt);
            }

            question.appendChild(optDiv);
            questions.appendChild(question);
        });
    } catch (error) {
        console.error("Error:", error);
    }
}

async function save_questions() {
    const username = getCookie("username");
    const authstr = build_authstr();

    const headers = new Headers();
    headers.set("Authorization", authstr);

    try {
        const questions = document.getElementById("questions");
        const qs = Array.from(questions.children);
        const s = await get_students_with_email(username);

        qs.forEach(async (q) => {
            const sid = s[0].id;
            const qid = q.dataset.qid;
            const opts = q.querySelector(".optDiv").querySelectorAll("select");
            let opt1;
            let opt2;
            let opt3;

            if (opts[0] === undefined) {
                opt1 = null;
            } else {
                opt1 = opts[0].value;
            }

            if (opts[1] === undefined) {
                opt2 = null;
            } else {
                opt2 = opts[1].value;
            }

            if (opts[2] === undefined) {
                opt3 = null;
            } else {
                opt3 = opts[2].value;
            }

            // answer object
            const a = {
                sid: parseInt(sid),
                qid: parseInt(qid),
                opt1: parseInt(opt1) || null,
                opt2: parseInt(opt2) || null,
                opt3: parseInt(opt3) || null,
            };

            const res = await post_answer(a);

            if (!res.ok) {
                alert("Antworten konnten nicht gespeichert werden. ⚠️");
            } else {
                alert("Antworten erfolgreich gespeichert! 🎉")
            }
        })
    } catch (error) {
        console.error("Error:", error);
    }
}
