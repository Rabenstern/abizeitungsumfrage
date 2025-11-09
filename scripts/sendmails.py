"""
Send automatic email to give login info
"""
import re
import smtplib
import sqlite3
import ssl
import dotenv
import toml

# Hello, here is your token: {token}
with open("sendmail.txt", "r", encoding="utf-8") as f:
    message = f.read()
with open("../config.toml", "r", encoding="utf-8") as f:
    config = toml.load(f)
from_address = dotenv.get_key("../.env", "ADMIN_EMAIL_ADDR")
password = dotenv.get_key("../.env", "ADMIN_EMAIL_PW")

# regex result
regres = re.search(
    r"(?:sqlite://)?(?P<path>.*?(?=\?|$))(?:\?mode=[^ ]+)",
    config["db"]["database_url"]
)
db_url = regres.group("path")

smtp_server = dotenv.get_key("../.env", "SMTP_SERVER")

# subject to change (see config)
conn = sqlite3.connect("../database.db")
cur = conn.cursor()

cur.execute("SELECT email, token FROM student")
data = cur.fetchall()

cur.close()
conn.close()

context = ssl.create_default_context()
with smtplib.SMTP_SSL(smtp_server, 465, context=context) as server:
    server.login(from_address, password)
    for (email, token) in data:
        server.sendmail(
            from_address,
            email,
            message.format(token=token),
        )
