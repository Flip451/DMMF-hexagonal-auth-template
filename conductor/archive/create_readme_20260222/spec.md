# Specification: README.md の作成

## Overview
本プロジェクトのビジョン、技術スタック、アーキテクチャ、および使用方法を網羅した包括的な `README.md` を作成します。日本語と英語の併記を行い、国内外の開発者がプロジェクトの意図を理解し、迅速に開発を開始できるようにします。

## Functional Requirements
- **バイリンガル対応:** 全セクションを日本語と英語で記述する。
- **アーキテクチャの視覚化:** 
    - DMMF、ヘキサゴナル、叫ぶアーキテクチャの適用について図（Mermaid等）または文章で詳細に解説する。
    - `libs/` と `apps/` の分離、および `api` から `domain` への依存排除といった最新の構成変更を反映する。
- **セットアップガイド:** Docker Compose および `cargo-make` を使用した環境構築手順を明文化する。
- **ディレクトリ構造解説:** 各パッケージの責務を一覧化する。
- **開発ワークフロー:** Conductor フレームワークや TDD に基づく開発サイクルを解説する。
- **ロードマップ:** 
    - Reliable Infrastructure Pattern (Outbox & Audit Columns)
    - 管理画面 API / UI
    - ユーザーのライフサイクル（状態遷移）の導入

## Acceptance Criteria
- [ ] 日本語と英語の両方で主要セクションが完備されていること。
- [ ] 最新のディレクトリ構造（`apps/server`, `apps/api`, `libs/usecase` 等）と整合していること。
- [ ] 紹介されているセットアップコマンドが実際に動作すること。
