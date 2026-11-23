---
name: develop-game-logic
description: GameLogicの決定的戦闘計算・複合Skill・ステータス計算を変更する際に使用する。
---

# GameLogicの変更

以下のパスはリポジトリルート基準。隣接する `../Specifications` の最新仕様と実装を確認してから変更する。参照先がない場合は仕様の所在を確認し、READMEやこのskillを仕様の代用にしない。

## 参照と実装境界

Specifications/specification/game/ 配下の対象計算仕様 を対象機能に応じて読む。通信型を変更するときはSpecificationsのprotoと各利用側のコピー・生成設定を照合する。

- このcrateはI/Oを持たない共有ライブラリ。 `src/lib.rs` の公開型とClient/GameServerの利用箇所を確認してから型を変更する。
- `battle/` は戦闘順序と行動、`skill.rs` / `composite_skill.rs` はSkill、`tactics/` はTactics、`pseudorandom.rs` は乱数。時刻や外部乱数への依存を計算経路へ入れない。
- 同じ初期状態・seedで同じ結果になるよう、乱数を消費する回数と対象の順序を仕様に合わせる。可変HIT数と主効果・副効果の適用順に注意する。
- 計算変更は式・丸め・上限・境界値を仕様と照合し、計算式をコメントに記載する。既存の未対応効果を成功として黙って無視しない。
- CharacterのRarity型は既存利用側との互換性のため残っている。masterdataからの取得はGameServer側の外部供給境界で扱い、推定しない。
- GameServerでは本リポジトリがsubmoduleとして利用される。本体のcommitと利用側のgitlink更新は別作業。今回限りのpush許可を将来のpush許可と解釈しない。

## 検証と引継ぎ

既存テストが対象機能を検証する場合は実行し、`cargo build --offline --locked`、`cargo clippy --offline --locked -- -D warnings`、`cargo fmt --check` で確認する。依存未取得なら無断で外部取得せず、必要なものを示す。新規テストの作成は適用されるAGENTS.mdとユーザー指示に従う。

READMEの実装済み機能と未接続箇所を実コードに合わせる。運用環境が必要な未実施検証をビルド結果と区別して報告する。skill自体は依存追加、DB migration実行、pushやデプロイへの許可を与えない。
