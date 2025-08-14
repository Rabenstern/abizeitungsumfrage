# abizeitungsumfrage

## running the server

.env:

```
TOKEN_SALT="random string"
DB_URL="sqlite:///umfrage.db"
```

in shell:

```bash
python3 -m venv .venv            # create virtual environment
. .venv/bin/activate             # activate virtual environment
pip install -r requirements.txt  # install reqs
uvicorn app:app --reload    # run server at 127.0.0.1:8000
