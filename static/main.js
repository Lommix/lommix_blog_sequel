import BevyRunner from "./js/bevy_runner.js";

document.addEventListener("DOMContentLoaded", function () {
    const nav = document.querySelector(".nav");
    const links = this.querySelectorAll(".nav-link");
    links.forEach((link, i) => {
        link.addEventListener("click", () => {
            document.querySelectorAll(".nav-link").forEach((link) => link.classList.remove("active"));
            link.classList.add("active");
            nav.style.setProperty("--nav-left", `${i * 25}%`);
        });

        if (window.location.pathname == link.getAttribute("hx-push-url")) {
            link.classList.add("active");
            nav.style.setProperty("--nav-left", `${i * 25}%`);
        }
    });

	document.body.addEventListener("htmx:afterSwap", (ev) => {
		hljs.highlightAll();
	});
});
