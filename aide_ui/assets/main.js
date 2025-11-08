window.scrollToTop = () => {
    window.scrollTo({
        top: 0,
        behavior: 'smooth'
    });

    const container = document.getElementById('scroll-container');
    if (container) {
        container.scrollTo({
            top: 0,
            behavior: 'smooth'
        });
    }
}