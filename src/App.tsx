// import { useState } from "react";
// import reactLogo from "./assets/react.svg";
// import { invoke } from "@tauri-apps/api/tauri";
// import "./App.css";

import Board from '@asseinfo/react-kanban';
import '@asseinfo/react-kanban/dist/styles.css'
// Tauriが提供するinvoke関数をインポートする
import { invoke } from '@tauri-apps/api'

// かんばんボードに最初に表示するデータを作成する
const board = {
  columns: [
    {
      id: 0,
      title: 'バックログ',
      cards: [
        {
          id: 0,
          title: 'かんばんボードを追加する',
          description: 'react-kanbanを使用する'
        },
      ]
    },
    {
      id: 1,
      title: '開発中',
      cards: []
    }
  ]
}

// ボードを表す型定義
type TBoard = {
  columns: [TColumn];
}
// カラムを表す型定義
type TColumn = {
  id: number;
  title: string;
  cards: [TCard];
}
// カードを表す型定義
type TCard = {
  id: number;
  title: string;
  description: string | undefined;
}
// カードの移動元を表す型定義
type TMovedFrom = {
  fromColumnId: number;
  fromPosition: number;
}
// カードの移動先を表す型定義
type TMovedTo = {
  toColumnId: number;
  toPosition: number;
}
// カードの位置を表すクラス
class CardPos {
  columnId: number;
  position: number;
  constructor(columnId: number, position: number) {
    this.columnId = columnId;
    this.position = position;
  }
}
// カードの追加直後に呼ばれるハンドラ
async function handleAddCard(board: TBoard, column: TColumn, card: TCard) {
  const pos = new CardPos(column.id, 0);
  // IPCでCoreプロセスのhandle_add_cardを呼ぶ（引数はJSON形式）
  await invoke<void>("handle_add_card", { "card": card, "pos": pos })
};
// カードの移動直後に呼ばれるハンドラ
async function handleMoveCard(board: TBoard, card: TCard, from: TMovedFrom, to: TMovedTo) {
  const fromPos = new CardPos(from.fromColumnId, from.fromPosition);
  const toPos = new CardPos(to.toColumnId, to.toPosition);
  await invoke<void>("handle_move_card", { "card": card, "from": fromPos, "to": toPos })
}
// カードの削除直後に呼ばれるハンドラ
async function handleRemoveCard(board: TBoard, column: TColumn, card: TCard) {
  await invoke<void>("handle_remove_card", { "card": card, "columnId": column.id })
};

function App() {

  return (
    <>
      <Board
        // ボードの初期データ
        initialBoard={board}
        // カードの追加を許可（トップに「＋」ボタンを表示）
        allowAddCard={{ on: "top" }}
        // カードの削除を許可
        allowRemoveCard
        // カラム（カードのグループ）のドラッグをオフにする
        disableColumnDrag
        // 新しいカードの作成時、idに現在時刻の数値表現をセットする
        onNewCardConfirm={(draftCard: any) => ({
          id: new Date().getTime(),
          ...draftCard
        })}
        // // 新しいカードが作成されたら、カード等の内容をコンソールに表示する
        // onCardNew={console.log}
        // カード等の内容をターミナルに表示する
        onCardNew={handleAddCard}
        onCardDragEnd={handleMoveCard}
        onCardRemove={handleRemoveCard}
      />
    </>
  );
}

export default App;
