<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import type { MessageStruct } from "./myMessageStruct";

    let messagePosition: number;
    let messageContainer: HTMLElement | null = null;
    let fetch_message: MessageStruct[] = [];
    let message = "初期メッセージ";
    let message_index: number = 0;
    let message_array_number: number = 0;
    const messageSpeed = 1;
    const refreshInterval = 100;

    onMount(async () => {
        //  初回読み込み
        await fetchData();

        //  １分ごとにdata.jsを再読み込み
        setInterval(fetchData, 60000);
        animateScroll();
    });

    onMount(() => {
        messageContainer = document.querySelector(".message-container");

        messagePosition = window.innerWidth;
        if (messageContainer) {
            messageContainer.style.left = messagePosition + "px";
            messageContainer.style.whiteSpace = "nowrap";
            messageContainer.style.overflow = "hidden";
        }
    });

    function animateScroll() {
        if (messageContainer) {
            messagePosition -= messageSpeed;

            if (messagePosition + messageContainer.clientWidth < 0) {
                messagePosition =
                    window.innerWidth + messageContainer.clientWidth;

                if (message_array_number > 0) {
                    let data = fetch_message[message_index].message;
                    message = data.replace(/"/g, "");
                    message_index++;
                    if (message_index >= message_array_number) {
                        message_index = 0;
                    }
                }
            }

            messageContainer.style.left = messagePosition + "px";
            requestAnimationFrame(animateScroll);
        }
    }

    async function fetchData() {
        try {
            let response: string[] = await invoke("fetch_message");
            for (var value of response) {
                let data = JSON.parse(value);
                fetch_message.push(data);
            }
            message_array_number = response.length;

            //  データの変換
        } catch (error) {
            console.error("インヴォークエラー", error);
        }
    }

    function changeMessage() {}
</script>

<div class="message-container">
    {message}
</div>

<style>
    .message-container {
        padding: auto;
        font-size: 80px;
        position: fixed;
        bottom: 0;
        left: 0;
        white-space: nowrap;
        overflow: hidden;
        line-height: 110%;
        background-color: rgba(80, 80, 80, 0.5);
    }
</style>
