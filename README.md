# abizeitungsumfrage

This project is an online questionnaire for our *ABI-Zeitung*.
It provides a web interface with which you can answer questions after logging
in via email and a pre-generated token.
Due to personnel constraints we have strongly split the front-end
(static HTML, CSS, JS) residing in `static/` and the back-end (API and DB).
This is a hobby project and has **NO WARRANTY** whatsoever.
It is designed to work for our use case and our use case only.
Nonetheless we hope others might find the project useful too.
This being *free software (GPL v3)* forks are welcome and PRs are appreciated 💜.

## running the server

To populate the DB at startup
`students.csv` in the format `email,last_name,first_name`
and `teachers.csv` in the format `last_name,first_name` are required.

In addition you will need the following in your `.env`:

```
TOKEN_SALT="random string"
DB_URL="database.db"
```

To start the server run this in your shell:

```bash
RUST_LOG=trace cargo run  # runs server with trace level logging
```
