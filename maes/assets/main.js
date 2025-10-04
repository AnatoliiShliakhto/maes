window.splitterContainer = null;
window.isDragging = false;
document.addEventListener('mousemove', (e) => {
    if (!isDragging || !window.splitterContainer) return;

    try {
        let newWidth = (e.clientX / window.splitterContainer.offsetWidth) * 100;
        if (newWidth < 30)  {
            newWidth = 30
        } else if (newWidth > 70) {
            newWidth = 70
        }
        window.splitterContainer.style.gridTemplateColumns = `${newWidth}% 10px 1fr`;
    } catch (error) {
        console.error(error);
    }
});

document.addEventListener('mouseup', () => {
    window.isDragging = false;
});
