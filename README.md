# abizeitungsumfrage

## running the server

```bash
python3 -m venv .venv            # create virtual environment
. .venv/bin/activate             # activate virtual environment
pip install -r requirements.txt  # install reqs
uvicorn main:app --reload        # run server at 127.0.0.1:8000
```
