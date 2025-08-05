# TODOアプリ  
目的：　構造体、列挙型、トレイト、モジュール設計、所有権と借用の応用 について理解する  
使用ツール：　なし  
詳細：  
タスクの作成、タスクの削除、タスクの完了、タスクの表示を行うことができるツール  
GUIはeguiで実装し、データベースにはSQLiteを用いる
```rust
pub struct TodoItem {  
    pub id: u32,  
    pub title: String,  
    pub status: Status,  
}  
```

## 初期設定
SQLite を使うために Diesel(ORM) をインストール
```sh
cargo install diesel_cli --no-default-features --features sqlite
```

## Diesel
データベースの作成（ .env の DATABASE_URL に記載されたデータベースを作成）
```
diesel setup
```

マイグレーションの作成
```
diesel migration generate {migration-name}
```
