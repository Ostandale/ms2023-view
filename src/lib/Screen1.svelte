<script lang="ts">
    import Chart from "./Chart_Draw.svelte";
    import type { TargetStruct } from "./myTargetStruct";

    export let screenNum: number;

    let targetValue: TargetStruct = {
        product_name: "",
        monthly_target: 0,
        daily_target: 0,
    };

    function fetchTargetValue(event: any) {
        targetValue = event.detail.value;
        console.log(targetValue);
    }
</script>

<div class="screen1_custom1">
    <div class="title">
        <div>
            製品名：{targetValue.product_name}
        </div>
        <div>
            月次生産目標：{targetValue.monthly_target}　日次生産目標：{targetValue.daily_target}
        </div>
    </div>
    <div class="custom1">
        <Chart
            props1={screenNum}
            props2={"normal"}
            on:value={fetchTargetValue}
        />
    </div>
    <div class="custom2">
        <Chart props1={screenNum} props2={"total"} />
    </div>
</div>

<style>
    .screen1_custom1 {
        display: flex;
        align-content: space-around;
        flex-direction: column;
        justify-content: space-around;
        width: 100%;
        height: 100%;
        margin: 0;
        padding: 0;
        align-items: center;
    }
    .custom1 {
        display: flex;
        flex-direction: row;
        width: 100%;
        height: 50%;
    }
    .custom2 {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
        height: 50%;
        padding: 20px;
    }
    .title {
        width: 95%;
        height: 220%;
        background-color: #202060;
        border: 1px solid white;
        border-radius: 20px;
        margin: 10px auto;
        padding: 20px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        font-size: 90px;
        line-height: 120%;
    }
</style>
