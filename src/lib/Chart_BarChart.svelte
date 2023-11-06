<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { BarChartSimple } from "@carbon/charts-svelte";
    import "@carbon/charts-svelte/styles.css";
    import { onMount } from "svelte";

    let data: any;
    let options: any;
    onMount(async () => {
        //  初回読み込み
        fetchData();
        //  １分ごとにdata.jsを再読み込み
        setInterval(fetchData, 10000);
    });

    async function fetchData() {
        try {
            let responce: string[] = await invoke("fetch_spreadsheet_data", {
                fileName: "monthly_SRS1A_summary_graph_data.js",
            });
            //  データの変換
            let res_data = responce[0];
            data = JSON.parse(JSON.parse(res_data));

            let res_options = responce[1];
            options = JSON.parse(JSON.parse(JSON.parse(res_options)));
        } catch (error) {
            console.error("インヴォークエラー", error);
        }
    }
</script>

<div class="graph">
    {#if data}
        <BarChartSimple {data} {options} style="padding:1px;" />
    {:else}
        データ未受信
    {/if}
</div>

<style>
    .graph {
        width: 98%;
        height: 95%;
        background-color: #202060;
        border: 1px solid white;
        border-radius: 20px;
        padding: 20px;
    }
</style>
