<script lang="ts">
    import { LineChart } from "chartist";

    import "chartist/dist/index.css";

    let { points }: { points: { date: Date; rating: number }[] } = $props();

    let chart: any;

    $effect(() => {
        new LineChart(
            chart,
            {
                labels: points.map(
                    (x) => `${x.date.getDate()}.${x.date.getMonth()}.${x.date.getFullYear()-2000}`,
                ),
                series: [points.map((x) => x.rating)],
            },
            {
                fullWidth: true,
                chartPadding: {
                    right: 40,
                },
            },
        );
    });
</script>

<div bind:this={chart}></div>
