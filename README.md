# algorism
RUSTの勉強とアルゴリズムの勉強をするためのリポジトリです。

## RUSTの勉強
* TODOアプリ  
    目的：　構造体、列挙型、トレイト、モジュール設計、所有権と借用の応用 について理解する  
    使用ツール：　なし  
    詳細：  
    タスクの作成、タスクの削除、タスクの完了、タスクの表示を行うことができるツール  
    jsonファイルにtodoリストのデータを保存する
    ```rust
    pub struct TodoItem {  
        pub id: u32,  
        pub title: String,  
        pub status: Status,  
    }  
    ```

* 簡単なAPIサーバー  
    目的：　REST API設計、DB接続をRUSTで行ってみる  
    使用ツール：　Rocket  
    詳細：  
