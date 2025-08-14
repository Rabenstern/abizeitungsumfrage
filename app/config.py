"""
config.py
"""
import os
import sys
from dotenv import load_dotenv
import logging as log

# load dotenv
log.info("loading .env...")
if not load_dotenv():
    log.critical("failed to load environment variables")
    sys.exit(1)

TOKEN_SALT = os.getenv("TOKEN_SALT")
DB_URL = os.getenv("DB_URL")
