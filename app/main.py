"""
main.py
"""
import logging as log
from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

from .models import Base
from .api.routes import router
from .database import setup_students, setup_teachers, engine

# setup logging
log.basicConfig(
    level=log.DEBUG,  # Set the logging level
    # Log message format
    format='%(asctime)s - %(levelname)s : %(message)s',
    handlers=[
        log.FileHandler('server.log'),  # Log to a file
        log.StreamHandler()  # Log to console
    ],
    force=True
)

log.info("STARTING SERVER...")
log.info("==================")
log.info("setting up API...")

# setup API
app = FastAPI()

# create DBs
Base.metadata.create_all(engine)

# create student table
setup_students()

# create teacher table
setup_teachers()

# include API routes
app.include_router(router)


app.mount(
    "/",
    StaticFiles(directory="static", html=True),
    name="static"
)
