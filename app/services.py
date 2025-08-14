"""
services.py
"""
# from .models import User
from .schemas import Message
# from .database import SessionLocal


def greet(greeting: Message):
    print(greeting.message)
    return {"status": 200}

# async def create_user(user: UserCreate):
#     db: Session = SessionLocal()
#     db_user = User(username=user.username, email=user.email)
#     db.add(db_user)
#     db.commit()
#     db.refresh(db_user)
#     db.close()
#     return db_user
