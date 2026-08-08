# Database connection helper
import sqlite3


def get_connection():
    return sqlite3.connect('app.db')
