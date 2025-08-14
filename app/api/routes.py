"""
api/routes.py
"""
from fastapi import APIRouter
from ..schemas import Message
from ..services import greet

router = APIRouter()


@router.get("/api/hello")
def get_hello():
    return {"message": "Hello, World!"}


@router.post("/api/greet")
def post_greet(greeting: Message):
    greet(greeting)
