"""
database.py
"""
import sys
import logging as log
from hashlib import sha256
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
import pandas as pd

from .config import DB_URL, TOKEN_SALT
from .models import Student, Teacher

try:
    log.info(f"connecting to DB @ {DB_URL}...")
    engine = create_engine(url=DB_URL)
except Exception as e:
    log.critical(f"failed to connect: {e}")
    sys.exit(1)

SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)


def setup_students():
    log.info("creating student DB...")

    session = SessionLocal()

    n_students = session.query(Student).count()

    if n_students == 0:
        log.warning("student DB empty, reading from CSV...")

        students = pd.read_csv("students.csv")

        emails = students["email"].to_list()
        sirnames = students["sirname"].to_list()
        names = students["name"].to_list()

        log.info("adding students to DB...")
        for i, email in enumerate(emails):
            text = f"{email}{TOKEN_SALT}"
            token = sha256(text.encode()).hexdigest()

            student = Student(
                email=email,
                token=token,
                sirname=sirnames[i],
                name=names[i]
            )
            session.add(student)

        log.info("students added")
    else:
        log.warning(f"student DB already has data ({n_students} rows)")

    session.commit()
    session.close()


def setup_teachers():
    log.info("creating teacher DB...")

    session = SessionLocal()

    n_teachers = session.query(Teacher).count()

    if n_teachers == 0:
        log.warning("teacher DB empty, reading from CSV...")

        teachers = pd.read_csv("teachers.csv")

        emails = teachers["email"].to_list()
        sirnames = teachers["sirname"].to_list()
        names = teachers["name"].to_list()

        log.info("adding teachers to DB...")
        for i, email in enumerate(emails):
            teacher = Teacher(
                email=email,
                sirname=sirnames[i],
                name=names[i]
            )
            session.add(teacher)

        log.info("teachers added")
    else:
        log.warning(f"teacher DB already has data ({n_teachers} rows)")

    session.commit()
    session.close()
