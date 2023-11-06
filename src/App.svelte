<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Screen from "./lib/Screen1.svelte";
  import MessageScroll from "./lib/Message_Scroll.svelte";
  import type { ConfigStruct } from "./lib/myStruct";

  //  画面を変更する時間　ミリ秒で設定
  const CHANGE_SCREEN_TIME = 30000;

  let sheetStruct: ConfigStruct[];

  fetch_struct();
  //  最初の要素だけtrueで残りはfalseで埋めた配列を作成
  //  表示する画面数だけ必要になる
  let MAX_DISPLAY_SCREEN: number;
  let showScreenIndex = 0;
  let showScreen: any[];

  // ミリ秒で画面が変わる時間をカウント
  setInterval(switchScreens, CHANGE_SCREEN_TIME);

  async function fetch_struct() {
    try {
      //  グラフ表示用コンフィグデータ構造体を受け取る
      sheetStruct = await invoke("fetch_struct_data");
      MAX_DISPLAY_SCREEN = sheetStruct.length - 1;
      showScreen = Array(MAX_DISPLAY_SCREEN).fill(false);
      showScreen[showScreenIndex] = true;
    } catch (error) {
      console.error("コンフィグ構造体読み込みエラー");
    }
  }

  //  配列の要素を呼び出される度に１つづつずらす
  function switchScreens() {
    showScreenIndex++;
    if (showScreenIndex >= MAX_DISPLAY_SCREEN) {
      showScreenIndex = 0;
    }
  }
</script>

<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <script type="module" defer src="./index.js"></script>
</head>
<!-- <meta
  name="google-signin-client_id"
  content="630326421714-66iri050l6psed860ut27nos2p1g1ggi.apps.googleusercontent.com"
/> -->
<main class="container1">
  {#if sheetStruct !== undefined}
    <Screen screenNum={showScreenIndex} />
  {/if}
  <div>
    <MessageScroll />
  </div>
</main>

<style>
  .container1 {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    vertical-align: top;
    align-items: stretch;
    padding: 0px auto;
    margin: 0px auto;

    overflow: hidden;
  }
</style>
