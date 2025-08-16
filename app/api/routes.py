"""
api/routes.py
"""
import logging as log
from fastapi import APIRouter, Depends
from ..schemas import Message, AuthReq, StudentIdRes
from ..services import post_api, get_students_id
from ..auth import authenticate_user

router = APIRouter()


@router.get("/api")
def api_get_api():
    return {"message": "Hello, World!"}


@router.post("/api")
def api_post_api(greeting: Message):
    return post_api(greeting)


# @router.get("/api/students")
# def api_get_students(auth: AuthReq):
#     if authify(auth).status == 200:
#         # return students
#         return
#
#     # return error
#     return


@router.get("/api/students/{email}")
def api_get_students_id(email: str = Depends(authenticate_user)):
    log.info("using students endpoint")
    resp = get_students_id(email)
    return resp
