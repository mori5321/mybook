STM (Software Transactional Memory) = 楽観ロック

TL2アルゴリズム (Transactional Locking 2)

- 一定のメモリ区間(ストライプ)単位で管理する
- それぞれのストライプは排他ロックとデータのバージョン保存を行う変数(lock&version)がある


read-version, write-version, read-set, write-set


投機的実行
tx内をとりあえず実行してみる。ロックがとれなかったらabortする。
メリット: 合成可能
デメリット: 意識すべき制約が多い(副作用を含むな、2回以上実行される可能性がある = 冪等であれ)


書き込みトランザクションのアルゴリズム
1. global version-clock を ローカルのread-version にコピー
2. トランザクションの投機的実行(一回transaction内実行してみてロックを獲得できたら通す)
    - メモリ書き込みの場合は実際に書き込まず、write-setに書き込み先アドレスとデータを保存
3. write-set のロック
    - writeセット中のアドレスに対応するストライプのロックを獲得。ロックを獲得できない場合はabort
4. global version-clock の increment
    - インクリメント後のバージョンをwrite-versionに保存
5. read-set の検証
    - read-set中にあるストライプのバージョンが他のスレッドによってロックされていないかread-version以下であるかをチェック
    - そうでないならアボート
    - ただし read-version + 1 = write-version の場合はこのチェックはスキップ可能
6. コミットとリリース


global version-clock ってなに
