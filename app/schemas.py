"""
schemas.py
"""
from pydantic import BaseModel


class Message(BaseModel):
    message: str


class AuthReq(BaseModel):
    email: str
    token: str


class AuthRes(BaseModel):
    status: int
    description: str


class StudentIdReq(BaseModel):
    email: str


class StudentIdRes(BaseModel):
    status: int
    description: str
    id: int
