from flask import Flask
from flask import jsonify
from flask_cors import CORS

app = Flask(__name__)
CORS(app, supports_credentials=True)


@app.route('/')
def hello_world():
    user = {
        "name": "Jasper",
        "age": 30
    }
    return jsonify(user)

if __name__ == '__main__':
    app.run(host="0.0.0.0", port=8092)