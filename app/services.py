"""
services.py
"""
import logging as log
from fastapi import status

# from .models import User
from .schemas import Message, AuthReq, AuthRes, StudentIdReq, StudentIdRes
from .database import SessionLocal, Student


def post_api(greeting: Message):
    print(greeting.message)
    return {"status": status.HTTP_200_OK}


def get_students_id(email: str):
    session = SessionLocal()
    result = session.query(Student).with_entities(
        Student.id).filter(Student.email == email).first()

    id = result.id

    resp = StudentIdRes(
        status=status.HTTP_200_OK,
        description="id recovery successful",
        id=id
    )
    return resp

# async def create_user(user: UserCreate):
#     db: Session = SessionLocal()
#     db_user = User(username=user.username, email=user.email)
#     db.add(db_user)
#     db.commit()
#     db.refresh(db_user)
#     db.close()
#     return db_user
