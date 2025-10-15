import ApexCharts from 'apexcharts'

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