
-----

# 競技プログラミング用C++特化環境

C++での競技プログラミングをより快適に進めるための、ビルド、テスト、ライブラリ管理の機能を提供する開発環境です。

## 環境構築 🛠️

この開発環境をセットアップするための手順です。

### 1\. パッケージのインストール

ビルドに必要な基本的なツールをインストールします。

```bash
sudo apt update
sudo apt install build-essential cmake ninja-build
```

### 2\. Boostライブラリの追加

Boostライブラリをソースからビルドしてインストールします。

```bash
cd /tmp
wget https://archives.boost.io/release/1.88.0/source/boost_1_88_0.tar.gz
tar -zxvf boost_1_88_0.tar.gz
cd boost_1_88_0
chmod +x bootstrap.sh
./bootstrap.sh
sudo ./b2 install -j$(nproc --all)
```

### 3\. Rustのインストール

Rustのツールチェインをインストールします。

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

-----

## 使い方 🚀

### リポジトリのクローン

このリポジトリはサブモジュールを利用しているため、クローンする際は `--recurse-submodules` オプションを付けてください。

```bash
git clone --recurse-submodules git@github.com:ItsukiYoshida/cp-cplusplus.git
```

もしオプションを付けずにクローンした場合や、ブランチを切り替えた後は、以下のコマンドでサブモジュールを初期化・更新できます。

```bash
git submodule update --init --recursive
```

### テストケースの準備

`test` ディレクトリ以下に、問題ごとのテストケース（入力ファイルと期待される出力ファイル）を配置します。

**ディレクトリ構成例:**

```plaintext
test/
├── A/
│   ├── A1      # A問題のテストケース1 (入力)
│   ├── A1.ans  # A問題のテストケース1 (正解出力)
│   ├── A2      # A問題のテストケース2 (入力)
│   └── A2.ans  # A問題のテストケース2 (正解出力)
├── B/
│   ├── B1
│   └── B1.ans
...
```

準備ができたら、以下のコマンドでGoogle Test用のソースコードを自動生成します。

```bash
python3 make_test.py
```

### ビルドとテストの実行

1.  **CMakeの実行 (初回のみ)**
`a` の部分を対象の問題（`b`, `c`...）に置き換えて実行してください。このコマンドは、`CMakeLists.txt`等を変更しない限り、一度実行すればOKです。

```bash
cmake --preset a
```

> [!NOTE]
> `CMakeLists.txt`や`CMakePresets.json`を書き換えた場合は、`--fresh`オプションを付けて再実行してください。
> ```bash
> cmake --preset a --fresh
> ```

2.  **ビルド**
対象の問題をビルドします。

```bash
cmake --build --preset a
```

3.  **テスト**
作成したテストケースを用いてテストを実行します。

```bash
ctest --preset a
```

> [!TIP]
> 環境変数 `GTEST_COLOR=1` を設定しておくと、テスト結果の出力がカラー表示になり見やすくなります。
> `export GTEST_COLOR=1`

### 自動ソースコード展開 (Auto Expander)

ビルド時に、インクルードしているユーザ定義ヘッダ (`#include "hoge.hpp"`) の内容が自動的にソースコードに展開されます。展開後のソースは `generated/` ディレクトリに生成されます。

この機能を無効にしたい（展開してほしくない）ヘッダファイルは、システムインクルード (`#include <hoge.hpp>`)として読み込んでください。

### エイリアス `cpc`

`bash` を利用している場合、`source .init` を実行すると便利なエイリアス `cpc` が使えるようになります。日常的な操作は `cpc` を経由することを推奨します。

`cpc` はソースコードやテストケースの変更を自動で検知するため、**ビルドやテスト生成コマンドを毎回実行する必要がありません**。

**主なコマンド:**

  * `cpc help`: ヘルプを表示します。
  * `cpc build [problem]`: 問題をビルドします。(`[problem]`は a, b, c...)
  * `cpc run [problem]`: 問題をビルドして実行します。
  * `cpc test [problem]`: 問題のテストを実行します。

例えば、A問題のテストを実行したい場合は、以下のコマンドだけで大丈夫です。

```bash
cpc test a
```

> [!WARNING]
> **[将来的な変更]**
> この `cpc` エイリアスは、将来的に別のコマンドラインツールに置き換えられる予定です。

-----

## 開発者向け情報 🧑‍💻

### 実行用コンテナ

すべての依存関係をインストール済みのDockerコンテナを[GitHub Packages](https://github.com/ItsukiYoshida/cp-cplusplus/packages)で公開しています。必要に応じて利用してください。

このコンテナはCIパイプラインでも利用しているため、不必要な更新は避けてください。

### 依存関係の追加

新たにライブラリなどの依存関係を追加した場合は、`.github/docker/Dockerfile` を更新してください。

`.github/docker/Dockerfile` または `.github/workflows/docker-build.yml` を変更してmainブランチにpushすると、GitHub Actionsによってコンテナイメージが自動的にリビルドされます（所要時間: 約15〜20分）。
