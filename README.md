# 赤錆

## 概要

赤錆は、 Rust で書かれたコンピューターのシミュレーターです。
柔軟に拡張可能なアーキテクチャを持ち、様々なコンピューターをシミュレートすることができます。

## 名前の由来

Rust は **錆** を意味します。
このプロジェクトは、 Rust を用いてコンピューターの深いところまで侵入していくため、深く侵食する **赤錆** にちなんで名付けられました。

## 要素

赤錆は以下の要素を再現します。

- ハードウェア
  - メモリ
  - 命令デコーダー
  - レジスタ
  - ALU
  - CPU
- ソフトウェア
  - アセンブリ言語
  - アセンブラ
  - コンパイラ

言語実行を行うための仮想マシンとして、ハードウェアを再現します。
再現の度合いは、選択可能にし、命令デコーダーを用いることも、
実際のハードウェアを用いることもできます。
実行可能な言語は全て自身で定義したものです。

これらの要素の根本的な設計を利用することで、任意の構成を実現することができます。

### アーキテクチャ

トレイトを用いて、アーキテクチャを定義します。
トレイトによって必要な挙動を定義し、それを用いることで、自由に好きな実装を試すことができます。

実際のハードウェアを操作できる実装を用意すれば、他の実装と同じように扱うことができます。

## 方針

全ての実装を Rust で行います。
また、クレートの使用は最小限に抑え、可能な限り自前で実装します。

型システムを活用し、コンパイル時にエラーを検知することで、実行時のエラーを減らします。
速度は追求せず、安全性と拡張性を重視します。

ステップ実行を行えるようにし、内部を観察することができるようにします。

並行処理時の競合を自由に起こすことができるようにし、再現性の低い状態を再現できるようにします。
(優先度低)

## 目的

コンピューターの内部を理解するための教材として利用できるようにします。

新たなコンピューターを設計する際に、その動作を確認するために利用できるようにします。
