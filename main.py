"""
main.py

server
"""
import os
import sys
import logging as l
from hashlib import sha256
from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from pydantic import BaseModel
import sqlalchemy as db
from sqlalchemy.ext.declarative import declarative_base
import pandas as pd
from dotenv import load_dotenv

# SETUP LOGGING ###
l.basicConfig(
    level=l.DEBUG,  # Set the logging level
    format='%(asctime)s - %(levelname)s : %(message)s',  # Log message format
    handlers=[
        l.FileHandler('server.log'),  # Log to a file
        l.StreamHandler()  # Log to console
    ]
)

# LOAD DOTENV ###
load_dotenv()
TOKEN_SALT = os.getenv("TOKEN_SALT")

# SETUP DATABASE ###
try:
    l.info("Connecting to DB...")
    engine = db.create_engine(url="sqlite:///umfrag.db")
except Exception as e:
    l.critical(f"Failed to connect: {e}")
    sys.exit(1)

Base = declarative_base()


class Student(Base):
    __tablename__ = 'students'

    id = db.Column(db.Integer, primary_key=True, autoincrement=True)
    email = db.Column(db.String)
    token = db.Column(db.String)

    def __repr__(self):
        return f"<Student(id='{self.id}', email='{self.email}', token='{self.token}'>"


Base.metadata.create_all(engine)

Session = db.orm.sessionmaker(bind=engine)
session = Session()

count = session.query(Student).count()

if count == 0:
    l.warning("DB empty. Reading from CSV...")

    students = pd.read_csv("students.csv")

    emails = students["email"].to_list()
    tokens = students["token"].to_list()

    l.info("Adding students to DB...")
    for i, email in enumerate(emails):
        text = f"{email}{TOKEN_SALT}"
        token = sha256(text.encode()).hexdigest()

        student = Student(email=email, token=token)
        session.add(student)

    l.info("Students added")
else:
    l.info(f"DB already has data ({count} rows)")

session.commit()
session.close()


# SETUP API ###
app = FastAPI()


class Message(BaseModel):
    message: str


@app.get("/api/hello")
def get_hello():
    return {"message": "Hello, World!"}


@app.post("/api/greet")
def post_greet(greeting: Message):
    print(greeting.message)
    return {"status": 200}


app.mount(
    "/",
    StaticFiles(directory="static", html=True),
    name="static"
)
