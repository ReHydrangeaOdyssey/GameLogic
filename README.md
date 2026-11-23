# GameLogic

ReHydrangeaOdysseyのゲーム再現プロジェクトを構成する共有戦闘ロジックライブラリです。仕様の正本は[Specifications](https://github.com/ReHydrangeaOdyssey/Specifications)です。

## 実装済みの機能・基盤

- seedを指定した擬似乱数と決定的な戦闘計算。
- Character・編成・Follower・Ability・状態異常・Tacticsの型と計算処理。
- 主効果と複数副効果を順に扱う複合Skill、可変HIT数、攻撃／防御の複合効果。
- ClientとGameServerから利用するRustライブラリ。

## 現在の制限

新仕様の特殊Skill・UF・一部効果への対応は未完了です。公開型には既存利用側との互換性を維持した部分があります。HTTP・DB・masterファイル読込は本リポジトリの責務に含みません。

## 開発

リポジトリルートで次を実行します。未取得の依存がある場合、offline実行は失敗します。

```sh
cargo build --offline --locked
cargo clippy --offline --locked -- -D warnings
cargo fmt --check
```

公開インターフェースは [src/lib.rs](src/lib.rs) を参照してください。

今後の実装作業向けskill: [develop-game-logic](.agents/skills/develop-game-logic/SKILL.md)。実装済みの範囲はソースと最新仕様を併せて確認してください。
