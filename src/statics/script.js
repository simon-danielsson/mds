document.addEventListener("DOMContentLoaded", () => {
    const sections = [...document.querySelectorAll(".section")];
    const presentation = [];

    sections.forEach((section, sectionNumber) => {
        const slides = [...section.querySelectorAll(":scope > .slide")];
        slides.forEach((slide, slideNumber) => {
            presentation.push({
                element: slide,
                sectionTitle: section.dataset.title,
                slideTitle: slide.dataset.title,
                sectionNumber,
                slideNumber
            });
        });
    });

    let current = 0;

    function updateProgress() {
        const progress = ((current + 1) / presentation.length) * 100;
        document.querySelector(".progress-bar").innerHTML = `
            <div class="progress-container">
                <div class="progress-fill" style="width:${progress}%"></div>
                <div class="progress-text">
                    ${presentation[current].sectionTitle}
                    &nbsp;&nbsp;│&nbsp;&nbsp;
                    ${presentation[current].slideTitle}
                    &nbsp;&nbsp;${current + 1}/${presentation.length}
                </div>
            </div>
        `;
    }

    function showSlide(index) {
        index = Math.max(0, Math.min(index, presentation.length - 1));
        presentation.forEach(item => {
            item.element.style.display = "none";
        });
        presentation[index].element.style.display = "block";
        current = index;
        updateProgress();
    }

    document.addEventListener("keydown", e => {
        if (e.key === "ArrowRight") {
            showSlide(current + 1);
        }
        if (e.key === "ArrowLeft") {
            showSlide(current - 1);
        }

    });

    showSlide(0);

});
