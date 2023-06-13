const para = document.querySelectorAll(".T");
const headingOne = document.querySelector("#H_ONE");
const skipBtn = document.querySelector("#skip__btn");
const loginSection = document.querySelector("#login");

if (para.length > 0) {
    const paraOneObserver = new IntersectionObserver((entries) => {
        entries.forEach((entry) => {
            if (entry.isIntersecting) {
                entry.target.classList.add("appear");
            } else {
                entry.target.classList.remove("appear");
            }
        });
    });

    para.forEach((para) => {
        paraOneObserver.observe(para);
    });
}

if (headingOne) {
    const headingOneObserver = new IntersectionObserver((entries) => {
        entries.forEach((entry) => {
            if (entry.isIntersecting) {
                entry.target.classList.add("pop");
            } else {
                entry.target.classList.remove("pop");
            }
        });
    });

    headingOneObserver.observe(headingOne);
}

if (skipBtn && loginSection) {
    const loginSectionObserver = new IntersectionObserver(
        (entries) => {
            entries.forEach((entry) => {
                if (entry.isIntersecting) {
                    skipBtn.classList.add("hide");
                } else {
                    skipBtn.classList.remove("hide");
                }
            });
        },
        {
            threshold: 0.5,
        }
    );

    loginSectionObserver.observe(loginSection);
}
