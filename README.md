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

## Prerequisites

* `stable` [Rust](https://rust-lang.org) toolchain

* `build-essential`
* `libssl-dev`
* `pkg-config`

## running the server

To populate the DB at startup the following files are required:

* `students.csv` in the format `email,last_name,first_name`
* `teachers.csv` in the format `last_name,first_name`
* `questions.csv` in the format `q,opt1,opt2,opt3`

An example for `questions.csv` would be:

```csv
q,opt1,opt2,opt3
"Bei jeder Party dabei",Student,,
"Süßestes Paar",Student,Student,
"Beste Kombi im Unterricht",Student,Teacher,
```

Note the extra `,` to indicate a `NULL` value for unused opts.

You will also need a `config.toml` with the following parameters:

```toml
title = "Meine Schule Jahrgang '26"

[files]
students_file = "students.csv"
teachers_file = "teachers.csv"
question_file = "questions.csv"

[db]
database_url = "sqlite://./database.db?mode=rwc" # if unsure keep this the way it is
```

In addition you will need the following in your `.env`:

```
TOKEN_SALT="random string"
ADMIN_EMAIL_ADDR="example@email.org"
ADMIN_EMAIL_PW="password"
SMTP_SERVER="smtp.email.org"
```

To start the server run **one** of these in your shell:

```bash
./run.sh      # for testing
./run.sh -d   # for detached process (nohup)
./run.sh -r   # for release build
./run.sh -dr  # for detached release build
```

If you wish to send emails to the participants automatically, you need to add the `scripts/sendmail.txt` file:

```txt
Hello, here's your token: {token}
```

An email will be sent to every student in the DB and `{token}` will be replaced with their token in the actual email.
