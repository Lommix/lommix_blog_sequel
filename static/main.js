import BevyRunner from "./js/bevy_runner.js";


document.addEventListener("DOMContentLoaded", function() {
	// forms
    document.querySelectorAll("form").forEach((form) => {
        form.addEventListener("submit", function (e) {
            e.preventDefault();
            const submit_button = form.querySelector("input[type=submit]");
            const action = form.getAttribute("action");

            const data = Array.from(new FormData(form)).reduce((acc, [key, value]) => {
                acc[key] = value;
                return acc;
            }, {});

            submit_button.disabled = true;

            fetch(action, {
                headers: {
                    "Content-Type": "application/json",
                },
                method: "POST",
                body: JSON.stringify(data),
            }).then((res) => {
                if (res.status !== 200) {
					setTimeout(() => {
						submit_button.disabled = false;
					}, 1000);
                } else {
                    submit_button.style.display = "none";
                    form.reset();
                    setTimeout(() => {
                        submit_button.style.display = "block";
                        submit_button.disabled = false;
                    }, 5000);
                }
            });
        });
    });
});
