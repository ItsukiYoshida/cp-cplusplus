# Require
```
sudo apt update
sudo apt install build-essential cmake ninja-build
```

Boostの追加
```
cd /tmp
wget https://archives.boost.io/release/1.88.0/source/boost_1_88_0.tar.gz
tar -zxvf boost_1_88_0.tar.gz
cd boost_1_88_0.tar.gz
chmod +x bootstrap.sh
./bootstrap.sh
sudo ./b2 install -j$(nproc --all)
```

# How To Clone
絶対に下のコマンドでクローンするんやで．これでクローンしてなくて動かないとか言うなよ．
```
git clone --recurse-submodules git@github.com:ItsukiYoshida/cp-cplusplus.git
```
もし，間違えてオプションなしでクローンした間抜けがいた場合は下のコマンドを打つんや．
```
git submodule update --init --recursive
```

# How To Use
test以下に各test caseのinput, outputを記述します．
複数の入出力にも対応しています．必ず，inputから始まるテキストファイルと，inputをoutputに置き換えたテキストファイルを用意してください．

```
input_hoge.txt <- OK
output_hoge.txt <- OK
input_huga.txt <- NG output_huga.txtが必要．無視される．
output_piyo.txt <- NG input_piyo.txtが必要．無視される．
foo_input.txt <- NG inputから始まっていない．無視される．
```

```
python3 make_test.py
```
でtest caseからGoogle Testを自動生成します．

```
cmake --preset a
cmake --build --preset a
```
でA問題をビルドできます．他の問題にする場合は適切に`a`を切り替えてください．
なお，`cmake --preset a`は一回で大丈夫です．
`CMakeLists.txt`や`CMakePresets.json`を書き換えたなどで，再度cmakeが必要な場合は必ず`cmake --preset a --fresh`を実行してください．

```
ctest --preset a
```
でA問題をテストできます．他の問題にする場合は適切に`a`を切り替えてください．
