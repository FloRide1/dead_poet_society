from flask import Flask, jsonify, abort
import psycopg2.extras
import psycopg2
import os
from dotenv import load_dotenv


load_dotenv()

app = Flask(__name__)

conn = psycopg2.connect(
    dbname = os.environ['POSTGRES_DB'],
    user =  os.environ['POSTGRES_USER'],
    password = os.environ['POSTGRES_PASSWORD'],
    host = os.environ['HOST'],
    port = os.environ['POSTGRES_PORT']
)
cur = conn.cursor()


@app.route('/')
def hello():
    return "Hello, Flask Dockerized!"

@app.route('/writer', methods=['GET'])
def get_writer():
     cursor = conn.cursor(cursor_factory = psycopg2.extras.RealDictCursor)
     cursor.execute('SELECT * FROM writer')
     res = cursor.fetchall()
     cursor.close()
     return jsonify(res)

@app.route('/writer/<int:id>', methods=['GET'])
def get_writer_by_id(id):
     cursor = conn.cursor(cursor_factory = psycopg2.extras.RealDictCursor)
     cursor.execute('SELECT * FROM writer WHERE id = %s', (id,))
     res = cursor.fetchone()
     cursor.close()
     if res:
         return jsonify(res)
     else:
         abort(404, 'Writer not found')

@app.route('/circle', methods=['GET'])
def get_circles():
     cursor = conn.cursor(cursor_factory = psycopg2.extras.RealDictCursor)
     cursor.execute('SELECT * FROM circle')
     res = cursor.fetchall()
     cursor.close()
     return jsonify(res)

@app.route('/circle/<int:id>', methods=['GET'])
def get_circle_by_id(id):
     cursor = conn.cursor(cursor_factory = psycopg2.extras.RealDictCursor)
     cursor.execute('SELECT * FROM circle WHERE id = %s', (id,))
     res = cursor.fetchone()
     cursor.close()
     if res:
         return jsonify(res)
     else:
         abort(404, 'Circle not found')

@app.route('/letter', methods=['GET'])
def get_letters():
     cursor = conn.cursor(cursor_factory = psycopg2.extras.RealDictCursor)
     cursor.execute('SELECT * FROM letter')
     res = cursor.fetchall()
     cursor.close()
     return jsonify(res)

@app.route('/letter/<int:id>', methods=['GET'])
def get_letter_by_id(id):
     cursor = conn.cursor(cursor_factory = psycopg2.extras.RealDictCursor)
     cursor.execute('SELECT * FROM letter WHERE id = %s', (id,))
     res = cursor.fetchone()
     cursor.close()
     if res:
         return jsonify(res)
     else:
         abort(404, 'Letter not found')

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5001)
