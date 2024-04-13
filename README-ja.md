<div align="center">[English](README.md)|[日本語](README-ja.md)</div>

<h1 align="center"> consowrap </h1>

consowrapは、コマンドラインツールを一手に管理し、コマンドを実行することができるシンプルなツールです。

<h2 align="center">前提</h2>

- コマンドラインツールを保存するディレクトリの作成と、コマンドラインツールの配置を行ってください。
- バイナリファイルの形では配布していません。使用する場合はビルドを行って下さい。

<h2 align="center">ビルド</h2>

Rustのインストールされた環境で以下を実行する。

```shell
cargo build --release
```

ビルドされた実行ファイルをディレクトリに配置してください。

ビルドされたファイルは以下のパスに生成されます。

```shell
./target/release/consowrap
```

### 実行権限の付与

配置したファイルに実行権限を付与して下さい。

```shell
sudo chmod +x consowrap
```

<h2 align="center">設定</h2>

### `.env`ファイルにコマンドラインツール保存ディレクトリを指定する。

`.env`ファイルに、管理したいコマンドラインツールのディレクトリパスを記載してください。

書式は以下のようになっています。

```.env
# 記述の書式
COMMAND_DRECTORY_PATH="your_commands_directory_path";

# 記載例
COMMAND_DRECTORY_PATH="./Commands";
```

また、`.env`ファイルが作成されていない場合は、自動で作成されます。

作成された`.env`ファイルに、コマンドラインツールのディレクトリパスを記載してください。


<h2 align="center">使い方</h2>

### コマンドを調べる

以下のコマンドで、使用可能なコマンド(`.env`に指定したディレクトリのファイル)の一覧が表示されます。

```shell
consowrap -l

# もしくは
consowrap --list
```

現在、`.env`に指定したディレクトリ内のファイルのみを表示するようになっているので、コマンドラインツール以外のものも表示される点に注意してください。

### コマンドを使用する

以下のような形でコマンドを実行できます。

```shell
consowrap command arg1 arg2

# optionも付与する場合
consowrap command -option arg1 arg2
```

### ヘルプ

困ったらヘルプを呼び出してください。

```shell
consowrap -h

# もしくは
consowrap --help
```

