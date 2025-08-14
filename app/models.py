"""
models.py
"""
from sqlalchemy import String, Integer
from sqlalchemy.orm import Mapped, mapped_column
from sqlalchemy.ext.declarative import declarative_base


Base = declarative_base()


class Student(Base):
    """
    student data structure
    """
    __tablename__ = 'students'

    id: Mapped[int] = mapped_column(
        Integer,
        primary_key=True,
        autoincrement=True
    )
    email: Mapped[str] = mapped_column(String)
    token: Mapped[str] = mapped_column(String)
    sirname: Mapped[str] = mapped_column(String)
    name: Mapped[str] = mapped_column(String)


class Teacher(Base):
    """
    teacher data structure
    """
    __tablename__ = 'teachers'

    id: Mapped[int] = mapped_column(
        Integer,
        primary_key=True,
        autoincrement=True
    )
    email: Mapped[str] = mapped_column(String)
    sirname: Mapped[str] = mapped_column(String)
    name: Mapped[str] = mapped_column(String)


class Q1(Base):
    """
    question with
    author id
    answered person id
    """
    __tablename__ = "Q1"

    id: Mapped[int] = mapped_column(
        Integer,
        primary_key=True,
        autoincrement=True
    )
    author: Mapped[int] = mapped_column(Integer)
    student: Mapped[int] = mapped_column(Integer)


class Q2(Base):
    """
    question with
    author id
    answered person 1 id
    answered person 2 id
    """
    __tablename__ = "Q2"

    id: Mapped[int] = mapped_column(
        Integer,
        primary_key=True,
        autoincrement=True
    )
    author: Mapped[int] = mapped_column(Integer)
    teacher1: Mapped[int] = mapped_column(Integer)
    teacher2: Mapped[int] = mapped_column(Integer)
