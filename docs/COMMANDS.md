# コマンド集

このドキュメントでは、nnmで使用可能なコマンドをまとめています。

## 基本コマンド

### `nnm init`

初期設定を行います。このコマンドは、設定ファイルを作成し、アプリケーションの初期状態を準備します。

設定ファイルの内容は[こちら](CONFIG.md)を参照してください。

```bash
nnm init
```

### `nnm`

設定ファイルのRSSリストからコンテンツを取得しコンソールに表示します。
初期状態では10件毎に取得します。

```bash
nnm
```

### `nnm rss add`

```bash
nnm rss add "[URL]"
```

RSSのURLを設定ファイルに登録します。

### `nnm rss delete`

```bash
nnm rss delete
```

RSSリストからURLを削除します。
インタラクティブモードが起動します。

### `nnm bookmark add`

```bash
nnm bookmark add "[URL]"
```

保存しておきたいURLをお気に入りリストに登録します。
後述の`nnm bookmark`で登録一覧を確認できます。

### `nnm bookmark delete`

```bash
nnm bookmark delete
```

お気に入りリストからURLを削除します。
インタラクティブモードが起動します。

### `nnm bookmark`

お気に入りに保存した一覧を確認します。

### `nnm history`

```bash
nnm history
```

過去に閲覧したコンテンツを表示します。
履歴の保存期間は [設定ファイル](CONFIG.md)の `history_expiration_day` に設定した値です。
