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
        hook_interaction();
    });
});

function hook_interaction() {
    document.querySelectorAll("[track]").forEach((e) => {
        e.addEventListener("click", track, { once: true });
    });
}

/** @param {HtmlEvent} event */
function track(el) {
    const action = find_action(el.target, 0);
    if (action === undefined) {
        return;
    }
    fetch("/htmx/interact", {
        method: "POST", // Specify the method to POST
        headers: {
            "Content-Type": "application/json", // Set the content type header for JSON
        },
        body: JSON.stringify({
            action: action,
        }),
    }).catch((error) => console.error("Error:", error));
}

/** @param{HtmlElement} element*/
function find_action(element, num) {
    if (num > 5) {
        return undefined;
    }
    return element.getAttribute("track") ?? find_action(element.parentElement, num + 1);
}
