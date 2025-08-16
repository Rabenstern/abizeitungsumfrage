import secrets
from fastapi import Depends, FastAPI, HTTPException, status
from fastapi.security import HTTPBasic, HTTPBasicCredentials

from .database import get_student_by_email

security = HTTPBasic()


def authenticate_user(credentials: HTTPBasicCredentials = Depends(security)):
    email = credentials.username
    student = get_student_by_email(email)

    # is_correct_username = secrets.compare_digest(credentials.username, )
    is_correct_password = secrets.compare_digest(
        credentials.password, student.token)

    if not is_correct_password:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Invalid credentials",
            headers={"WWW-Authenticate": "Basic"},
        )

    return credentials.username
