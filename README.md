Arbitrary Code
=====


任意コードの配布と実行  
wasmバイナリの配布とホストシェルコマンドの実行を行うサンプル  


実行
-----

ビルドとサーバの起動  
以下のパッケージが必要  
- pipenv pyenv [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```sh
$ cd server

# 配布する wasm プログラムのビルド
$ wasm-pack build assets/hello-world

# サーバ起動
$ pipenv run python src/server.py
```

クライアントアプリの起動

```sh
# サーバから wasm バイナリを取得し実行する
$ cargo run -- --program hello-world --remote
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/arbitrary-code --program hello-world --remote`
hello, Aofusa !
```


開発
-----

server/assets/hello-world を参考に  
entry_point関数内に任意のコードを記述することでクライアント上で実行することができます。  

クライアントマシン上でシェルの操作を行いたい場合
sh関数にコマンドを記載することで実行することができます。  

sh関数によるコマンドの実行はwasmではなくネイティブのRustアプリから  
クライアントマシンに対して命令が送られ実行されます。  

