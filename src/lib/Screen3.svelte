<script lang="ts">
    import Chart1 from "./Chart_Sre_Normal.svelte";
    import Chart2 from "./Chart_Sre_Total.svelte";
</script>

<div class="screen2_custom1">
    <div class="custom1">
        <Chart1 />
    </div>
    <div class="custom2">
        <Chart2 />
    </div>
</div>

<style>
    .screen2_custom1 {
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

    .custom11 {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 60%;
        height: 100%;
        padding: 20px auto;
    }
    .custom12 {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 40%;
        height: 100%;
        padding: 20px auto;
    }
    .custom2 {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
        height: 50%;
        padding: 20px;
    }
</style>
