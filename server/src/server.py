from flask import Flask
import os


app = Flask(__name__)


@app.route('/', methods=['GET'])
def route():
    return '\n'.join(os.listdir('assets'))


@app.route('/<program>', methods=['GET'])
def program(program):
    filepath = f'assets/{program}/pkg/{program.replace("-", "_")}_bg.wasm'
    try:
        with open(filepath, mode='rb') as f:
            return f.read()
    except FileNotFoundError as e:
        return f'please build {program}\n\nrun this command\n  $ wasm-pack build assets/{program}'


if __name__=='__main__':
    app.run(debug=True)

