import ApexCharts from 'apexcharts';
window.ApexCharts = ApexCharts;

window.splitterContainer = null;
window.isDragging = false;
document.addEventListener('mousemove', (e) => {
    if (!isDragging || !window.splitterContainer) return;

    const container = window.splitterContainer;
    const rect = container.getBoundingClientRect();

    let x = e.clientX - rect.left;
    x = Math.max(0, Math.min(x, rect.width));
    let leftPct = (x / rect.width) * 100;
    leftPct = Math.max(30, Math.min(leftPct, 70));

    container.style.gridTemplateColumns = `${leftPct}% 10px 1fr`;
});

document.addEventListener('mouseup', () => {
    window.isDragging = false;
});

window.assignSplitter = () => {
    window.splitterContainer = document.querySelector('#splitter-container');
    const splitter = document.querySelector('#splitter');
    splitter.addEventListener('mousedown', (e) => {
        window.isDragging = true;
        e.preventDefault();
    });
}

// Charts
function wrapByWords(value, maxLen) {
    if (typeof value !== 'string' || maxLen <= 0) return value;
    const lines = [];
    let line = '';

    for (const word of value.split(/\s+/).filter(Boolean)) {
        if (word.length > maxLen) {
            if (line) {
                lines.push(line);
                line = '';
            }
            for (let i = 0; i < word.length; i += maxLen) {
                lines.push(word.slice(i, i + maxLen));
            }
            continue;
        }

        if (!line) {
            line = word;
        } else if (line.length + 1 + word.length <= maxLen) {
            line += ' ' + word;
        } else {
            lines.push(line);
            line = word;
        }
    }

    if (line) lines.push(line);
    return lines;
}

function getColors(count, distributed = false) {
    const paletteA = ['#008FFB'];
    const paletteB = ['#00E396', '#FF4560'];
    const paletteC = ['#00E396', '#FEB019', '#FF4560'];
    const paletteD = ['#00E396', '#FFEB3B', '#FEB019', '#FF4560'];
    const paletteE = ['#008FFB', '#00E396', '#FFEB3B', '#FEB019', '#FF4560'];

    if (!distributed) return paletteA;
    if (count === paletteE.length) return paletteE;
    if (count === paletteD.length) return paletteD;
    if (count === paletteC.length) return paletteC;
    if (count === paletteB.length) return paletteB;

    return paletteA;
}

const chartsCache = new Map();

function ensureVisibleSize(el) {
    const rect = el.getBoundingClientRect();
    return rect.width > 0 && rect.height > 0;
}

function waitForVisible(el, timeout = 2000) {
    return new Promise((resolve, reject) => {
        if (ensureVisibleSize(el)) return resolve();
        const obs = new ResizeObserver(() => {
            if (ensureVisibleSize(el)) {
                obs.disconnect();
                resolve();
            }
        });
        obs.observe(el);
        const t = setTimeout(() => {
            obs.disconnect();
            resolve();
        }, timeout);
    });
}

window.barChart = async (id, payload) => {
    const el = document.getElementById(id);
    if (!el) return;

    if (chartsCache.has(id)) {
        try { await chartsCache.get(id).destroy(); } catch {}
        chartsCache.delete(id);
    }

    await waitForVisible(el);

    const BAR_HEIGHT = 50;
    const PADDING_BETWEEN_BARS = 15;
    const AXIS_OVERHEAD = 40;
    const chartHeight =
        (payload.categories.length * BAR_HEIGHT) +
        (payload.categories.length * PADDING_BETWEEN_BARS) +
        AXIS_OVERHEAD;

    const palette = getColors(payload.categories.length, payload.distributed);

    const options = {
        series: payload.series,
        chart: {
            type: 'bar',
            height: chartHeight,
            width: 790,
            toolbar: { show: false }
        },
        grid: { show: false, padding: { left: 30, right: 0, top: 0, bottom: 0 } },
        plotOptions: {
            bar: {
                borderRadius: 4,
                borderRadiusApplication: 'around',
                horizontal: true,
                distributed: payload.distributed,
                barHeight: '80%',
            },
        },
        colors: palette,
        dataLabels: {
            enabled: true,
            style: { colors: ['black'] },
            formatter: (v) => v + '%',
        },
        xaxis: {
            axisBorder: { show: true },
            axisTicks: { show: false },
            labels: { show: false },
            categories: payload.categories,
            min: 0,
            max: 100,
        },
        yaxis: {
            labels: {
                maxWidth: 340,
                style: { fontSize: 13 },
                formatter: (value) => wrapByWords(String(value), 35),
            },
        },
        tooltip: { enabled: false },
        legend: { show: false },
    };

    const chart = new window.ApexCharts(el, options);
    chartsCache.set(id, chart);
    await chart.render();
};

window.stackedBarChart = async (id, payload) => {
    const el = document.getElementById(id);
    if (!el) return;

    if (chartsCache.has(id)) {
        try { await chartsCache.get(id).destroy(); } catch {}
        chartsCache.delete(id);
    }

    await waitForVisible(el);

    const BAR_HEIGHT = 50;
    const PADDING_BETWEEN_BARS = 15;
    const AXIS_OVERHEAD = 40;
    const chartHeight =
        (payload.categories.length * BAR_HEIGHT) +
        (payload.categories.length * PADDING_BETWEEN_BARS) +
        AXIS_OVERHEAD;
    const palette = getColors(payload.series.length, true);

    const options = {
        series: payload.series,
        chart: {
            type: 'bar',
            stacked: true,
            height: chartHeight,
            width: 790,
            toolbar: { show: false }
        },
        grid: { show: false, padding: { left: 30, right: 0, top: 0, bottom: 0 } },
        plotOptions: {
            bar: {
                borderRadius: 4,
                borderRadiusApplication: 'end',
                horizontal: true,
                distributed: false,
                barHeight: '80%',
            },
        },
        colors: palette,
        dataLabels: {
            enabled: true,
            style: { colors: ['black'] },
            formatter: (v) => v + '%',
        },
        xaxis: {
            axisBorder: { show: true },
            axisTicks: { show: false },
            labels: { show: false },
            categories: payload.categories,
            min: 0,
            max: 100,
        },
        yaxis: {
            labels: {
                maxWidth: 340,
                style: { fontSize: 13 },
                formatter: (value) => wrapByWords(String(value), 35),
            },
        },
        tooltip: { enabled: false },
        legend: { show: true },
    };

    const chart = new window.ApexCharts(el, options);
    chartsCache.set(id, chart);
    await chart.render();
};

window.dispatchEvent(new Event('charts-ready'));