### 使用技術
- rust
- actix-web
- sqlx(mysql)

## 実行方法
- 実行環境として、cargoをセットアップするか、セットアップせず実行ファイルを実行する手がある。（実行ファイルはfor linux:開発者環境はubuntu20.04 x86でビルド）

- cargo をセットアップする場合 （公式： https://doc.rust-jp.rs/book-ja/ch01-01-installation.html）
    - linux ではこのコマンドのみ　
        - $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    - 上記の環境を構築後、cargo runでmain.rsが起動(本リポジトリでは以降の準備が必要)
### DB
- mysql databaseを立ち上げる。
- 適当な名前のデータベースを作成する

### migration
- cargoを用いない場合は立ち上げたDBに対して　migrations/20221124104119_initial.sqlを実行する。
- cargoを用いる際は以下ツールのインストールとマイグレーションを行う。
- cargo install sqlx-cli
- sqlx migrate run
- migrastions/のhogehoge.sqlが実行される

### env
- rootに.envファイルを作成しデータベース接続に必要な以下の値を書き込む.
[database_name]はDBセットアップ時に付けたのデータベースの名前
    - DATABASE_URL=mysql://[user_name]:[password]@[address]/[database_name]
        - ex.) DATABASE_URL=mysql://muscle:muscle@localohost:3306/medical

- cargo run or（実行ファイルなら ./ [実行ファイル名]）


## design
- module分けとしては以下
    - domain
        - coreなモデルの定義
    - presentaition
        - リクエスト、レスポンスモデルの定義、ユースケース呼び出しメソッドを実装しており、以下を行う。FWやユースケースとドメインの結合を抑える
            - リクエストからのパラメータ抜き出し
            - インスタンスの初期設定、ユースケースの呼び出し
            - レスポンスモデルへの詰め替え
    - repository
        - データベースとの結合部
        - interfaceはdomainに配置し、repositoryで実装。
    - usecase
        - repositoryとdomainを使用してユースケースを組み立てる
    - utils
        - 共通して使われるものや、どこにも所属できない概念のものを格納
    - middleware
        - middleware的なもの
        - authn
            - token処理はここので行っている
- unit testはusecaseとdomainのみ作成した。各ファイルに記述
- usecaseのテストはrepositoryをmockオブジェクトに差し替えて正常系一部のみテストした.
- errorのハンドリングは主だったところ作ったが、あまり色々なケースに網羅的には対応していない。

## 想定など

- 医療なのでユーザー＝医者と裏設定する
- 今回はサンプルなので自己サインアップし、ユーザー登録する。tokenが返ってくるので、authorization headerとして他のAPIに仕込む。
- 医者と患者のリレーションを張ったが、権限制御等はないので、特に機能上役割を果たしていない。
    - 本来、自分の担当患者の情報しか見えてはいけないという状況を叶えるために想定した。
        - 患者に複数担当者がつくことも想定し、ユーザー:患者はn:nで結びつく。
    - 今回は取得できる患者に制約を実装していない（全て取ってくる）ので、自分の患者と自分のじゃない患者がrecordを見ればわかる程度。
- 問診情報とは症状と問診日を想定した。例えば熱、喉の痛み、頭痛など。これらが患者に1:nで結びつく。
    - この辺はレコードが増えることが想定されるので、インデックスをしっかり貼る方が良いが、今回は実装していない。
    - 診断者もわかるようにuser_idとも紐付ける。
    - 問診日とレコードの作成日は一致しないことも想定し、問診日を作成。
- 患者単体登録はできるが患者のいない問診情報というものは今回の想定だと意味不明なので、問診情報を先に登録することはできないとする。
    - できることは患者登録、患者を指定した問診登録、患者と問診同時登録の三つ
    - 患者登録後、患者を指定して問診情報を単体登録はできる。
    - 同時登録は実行順序を守るために患者登録と問診情報登録とは別APIとした。ormでトランザクション管理したい（commitを複数にしたくもないし、）が、ユースケースで順序担保した。患者が登録完了し、問診の登録で失敗してもデータ上は問題ないので、そこのロールバックはしない。
- 基本的にid,codeを識別子としてもつ。codeでユーザーから個体識別しなければならなそうなものはcodeを配置し、codeを元にやりとりする。idは外に出さない。
## 実行
- 各Apiとcurlの例を以下に記す.パラメータは適宜変更。公開APIは８つ
- 環境変数としてTOKENにjwt tokenを仕込むと楽
- サインアップ
    - curl "http://localhost:8000/api/user" -X POST -H "Content-Type:application/json" -d '{"name":"test_user","password":"12345678"}'
- サインイン
    - curl "http://localhost:8000/api/user/login" -X POST -H "Content-Type:application/json" -d '{"code":"01GJT4JH83TFDT0D0SDH8ZGSQH","password":"12345678"}'
- 患者登録
    - curl "http://localhost:8000/api/patient" -X POST -H "Content-Type:application/json" -H "Autorization:Bearer ${TOKEN}" -d '{"name":"test_patient"}'
- 患者一覧取得
    - curl "http://localhost:8000/api/patient" -X GET -H "Content-Type:application/json" -H "Autorization:Bearer ${TOKEN}"
- 患者指定の問診情報登録
    - curl "http://localhost:8000/api/medical_examination" -X POST -H "Content-Type:application/json" -H "Autorization:Bearer ${TOKEN}" -d '{"symptom":"headach","patient_code":"01GJT7PAVJ1VCTF4YDQMVQPJYA","interviewed_at":"2022-12-12T12:12:12+0900"}'
- 指定患者の問診情報取得
    - curl "http://localhost:8000/api/medical_examination?patient_code=01GJT7PAVJ1VCTF4YDQMVQPJYA" -X GET -H "Content-Type:application/json" -H "Autorization:Bearer ${TOKEN}"
- 患者と問診同時登録
    - curl "http://localhost:8000/api/patient/with_me" -X POST -H "Content-Type:application/json" -H "Autorization:Bearer ${TOKEN}" -d '{"name":"test_patient2","symptom":"feaver","interviewed_at":"2022-12-13T12:12:12+0900"}'
- 患者担当設定
    - curl "http://localhost:8000/api/user/assign" -X POST -H "Content-Type:application/json" -H "Autorization:Bearer ${TOKEN}" -d '{"patient_code":"01GJT4JH83TFDT0D0SDH8ZGSQH"}'