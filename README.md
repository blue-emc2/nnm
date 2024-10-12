# nnm

## インストール手順

以下の手順に従って、このコンソールアプリケーションをインストールしてください。

### 必要条件
- **オペレーティングシステム**: このアプリケーションは、以下のOSで動作確認されています:
  - macOS

### インストール方法

1. **バイナリのダウンロード**  
   アプリケーションの最新バージョンのバイナリを [リリースページ](https://github.com/blue-emc2/nnm-dev/releases) からダウンロードしてください。
      - macOS: `app.zip`

2. **ダウンロードしたファイルを解凍**  
   ダウンロードしたファイルを解凍します:
   - Linux/macOS:
     ```bash
     tar -xzf app.zip
     ```

3. **バイナリの移動**  
   解凍したバイナリを任意の場所に移動します:
   - Linux/macOS:
     ```bash
     sudo mv your-app /home/{user}/path/to
     ```

4. **実行権限の付与 (Linux/macOS)**  
   必要に応じて実行権限を付与します:
   ```bash
   chmod +x /home/{user}/path/to
   ```

5. **動作確認**  
   インストールが正しく完了したことを確認するために、次のコマンドを実行してください:
   ```bash
   ./nnm --V
   ```
   Macの場合、起動時に`開発元が不明なMacアプリを開く`ダイアログが表示される事があるので、開くを選択して再度コマンドを実行すると動きます。

7. **起動する**
   ```bash
   ./nnm
   ```
