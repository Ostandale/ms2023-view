<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { ComboChart } from "@carbon/charts-svelte";
    import "@carbon/charts-svelte/styles.css";
    import { createEventDispatcher } from "svelte";

    export let props1: number;
    export let props2: string;

    let lastProps1: number;
    let data: any;
    let options: any;
    let target: any;

    const dispatch = createEventDispatcher();

    $: {
        if (props1 !== lastProps1) {
            fetchData();
            lastProps1 = props1;
        }
        if (typeof target !== "undefined") {
            dispatch("value", { value: target });
        }
    }

    async function fetchData() {
        try {
            let response: string[] = await invoke("fetch_spreadsheet_data2", {
                configNum: props1,
                dataType: props2,
            });
            //  データの変換
            let res_data = response[0];
            data = JSON.parse(JSON.parse(res_data));

            let res_options = response[1];
            options = JSON.parse(JSON.parse(res_options));

            let res_target = response[2];
            target = JSON.parse(JSON.parse(res_target));
        } catch (error) {
            console.error("インヴォークエラー", error);
        }
    }
</script>

<div class="graph">
    {#if data}
        {#if props2 == "normal"}
            日次生産高　　日次目標：{target.daily_target}
        {:else if props2 == "total"}
            日次生産高　積算　　月次目標：{target.monthly_target}
        {/if}
        <ComboChart {data} {options} style="padding:1px;" />
    {:else}
        データ未受信
    {/if}
</div>

<style>
    .graph {
        width: 95%;
        height: 95%;
        background-color: #202060;
        border: 1px solid white;
        border-radius: 20px;
        margin: 10px auto;
        padding: 20px;
    }
</style>
