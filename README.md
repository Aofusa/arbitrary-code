Arbitrary Code
=====


任意コードの配布と実行  
wasmバイナリの配布とホストシェルコマンドの実行を行うサンプル  


実行
-----

ビルドとサーバの起動  

```sh
cd server

# 配布する wasm プログラムのビルド
wasm-pack build assets/hello-world

# サーバ起動
pipenv run python server.py
```

クライアントアプリの起動

```sh
# サーバから wasm バイナリを取得し実行する
cargo run
```

