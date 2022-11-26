### tool
- rust
- actix-web
- sqlx

## DB
- prepare mysql database
- set connection env below [env] section

### migration
- cargo install sqlx-cli
- sqlx migrate run

## env
- make .env file and write it down
    - DATABASE_URL=mysql://[user_name]:[password]@[address]/[database_name]
        - ex.) DATABASE_URL=mysql://muscle:muscle@172.17.0.3:3306/muscle
- make secret.ky at root directory

- cargo run 
- ./target/debug/[exe_file]



## pre

- 医療なのでユーザー＝医者と設定する
- あまりtoBではないが自己サインアップし、ユーザー登録する。
- 医者と患者のリレーションを張ったが、権限制御とうはないので、特に機能上役割を果たしていない。
    - 自分の患者と自分のじゃない患者がわかる程度
- 患者単体登録はできるが患者のいない問診情報というものは意味不明なので、問診情報を先に登録することはできない。
    - 患者登録後、患者を指定して問診情報を登録はできる

- サインアップ
- curl "http://localhost:8000/api/user/login" -X POST -H 'Content-Type:application/json' -d '{"name":"test_user","password":"12345678"}' -v;
- サインイン
- curl "http://localhost:8000/api/user/login" -X POST -H 'Content-Type:application/json' -d '{"name":"test_user","password":"12345678"}' -v;
- 患者情報一覧取得
- curl "http://localhost:8000/api/patient" -X GET -H 'Content-Type:application/json' -H 'Autorization:Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2Njk0NzIwNjMsImV4cCI6MTY2OTQ3NTY2MywidXNlcl9pZCI6IjAxR0pUNEpIODIySlpERFYyWTJHTk1LMFdGIn0.TQczGiYrP4JMOI5CU_lSoE-ThYx87UMMkgcNX0VR6o8' -v
- 患者登録
- curl "http://localhost:8000/api/patient" -X POST -H 'Content-Type:application/json' -H 'Autorization:Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2Njk0NzIwNjMsImV4cCI6MTY2OTQ3NTY2MywidXNlcl9pZCI6IjAxR0pUNEpIODIySlpERFYyWTJHTk1LMFdGIn0.TQczGiYrP4JMOI5CU_lSoE-ThYx87UMMkgcNX0VR6o8' -d '{"name":"test_patient"}' -v
- 患者担当設定
- 問診情報登録
- curl "http://localhost:8000/api/medical_examination" -X POST -H 'Content-Type:application/json' -H 'Autorization:Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2Njk0NzIwNjMsImV4cCI6MTY2OTQ3NTY2MywidXNlcl9pZCI6IjAxR0pUNEpIODIySlpERFYyWTJHTk1LMFdGIn0.TQczGiYrP4JMOI5CU_lSoE-ThYx87UMMkgcNX0VR6o8' -d '{"symptom":"headach","patient_id":"ss","interviewed_at":"2022-12-12T12:12:12+0900"}' -v
- 指定患者の問診情報取得
