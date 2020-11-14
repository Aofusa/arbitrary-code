Sample wasm Server
=====


wasm バイナリを返却するサンプルサーバ  


環境構築と実行  
-----

requirements  
- pipenv pyenv [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

サーバの起動  
```sh
# 環境構築
pipenv --python 3.7
pipenv install

# 配布する wasm プログラムのビルド
wasm-pack build assets/hello-world

# サーバの起動
pipenv run python server.py
```

wasmファイルの取得  
```sh
# wasm ファイルの取得（ローカルファイルを取得）
curl http://127.0.0.1:5000/hello-world

# 一覧の取得
curl http://127.0.0.1:5000
```

