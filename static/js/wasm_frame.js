class WasmFrame extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: "open" }); // Attach shadow root to the custom element
        this.shadowRoot.innerHTML = `
			<style>
			.wrapper {
				aspect-ratio: 16/9;
				position: relative;
				overflow: hidden;
				outline: 0.125rem solid #FFFFFF;
				border-radius: 0.25rem;
			}
			#wasm-loader{
				position:relative;
				width: 100%;
				height: 100%;
			}
			#wasm-frame{
				display: none;
				width: 100%;
				height: 100%;
				overflow: hidden;
				border: none;
			}
			.centered-abs{
				position: absolute;
				top: 50%;
				left: 50%;
				transform: translate(-50%, -50%);
			}
			#load-button{
				font-size: 2rem;
				padding: 1rem 2rem;
				background-color: green;
				color: white;
				border: none;
				border-radius: 0.25rem;
				cursor: pointer;
				box-shadow: -1rem 1rem 3rem 0 #000;
			}
			#load-button:hover{
				background: greenyellow;
			}
			#fullscreen-button{
				display: none;
				z-index: 100;
				position: absolute;
				right: 0.5rem;
				bottom: 0.5rem;
				background: none;
				border: none;
				cursor: pointer;
			}
			@keyframes puls {
				0% { transform: blur(0.5rem); }
				50% { transform: blur(0.8rem); }
				100% { transform: blur(0.5rem); }
			}
			</style>
			<div class="wrapper">
				<iframe id="wasm-frame" src=""></iframe>
				<button id="fullscreen-button">
					<svg width="48px" height="48px" stroke-width="1.5" viewBox="0 0 21 21" xmlns="http://www.w3.org/2000/svg" fill="#FFF" style="--darkreader-inline-fill: #FFF;" data-darkreader-inline-fill=""><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <g fill="none" fill-rule="evenodd" stroke="#FFF" stroke-linecap="round" stroke-linejoin="round" transform="translate(2 2)" style="--darkreader-inline-stroke: #e8e6e3;" data-darkreader-inline-stroke=""> <path d="m16.5 5.5v-4.978l-5.5.014"></path> <path d="m16.5.522-6 5.907"></path> <path d="m11 16.521 5.5.002-.013-5.5"></path> <path d="m16.5 16.429-6-5.907"></path> <path d="m.5 5.5v-5h5.5"></path> <path d="m6.5 6.429-6-5.907"></path> <path d="m6 16.516-5.5.007v-5.023"></path> <path d="m6.5 10.5-6 6"></path> </g> </g></svg>
				</button>
				<div id="wasm-loader">
					<img src="" alt="WasmFrame" id="wasm-cover">
					<a class="centered-abs" id="load-button">Load & Play</a>
				</div>
			</div>
		`;
    }
    connectedCallback() {
        this.shadowRoot.getElementById("load-button").onclick = () => this.play();
        this.shadowRoot.getElementById("fullscreen-button").onclick = () => this.fullscreen();
        this.shadowRoot.getElementById("wasm-cover").src = this.getAttribute("cover");
    }

    play = () => {
        this.shadowRoot.getElementById("wasm-loader").style.display = "none";
        this.shadowRoot.getElementById("wasm-frame").style.display = "block";
        const wasmFrame = this.shadowRoot.getElementById("wasm-frame");
        wasmFrame.src = this.getAttribute("src");
        this.shadowRoot.getElementById("fullscreen-button").style.display = "block";
    };

    fullscreen = () => {
        const iframe = this.shadowRoot.getElementById("wasm-frame");
        if (iframe.requestFullscreen) {
            iframe.requestFullscreen();
        } else if (iframe.msRequestFullscreen) {
            iframe.msRequestFullscreen();
        } else if (iframe.mozRequestFullScreen) {
            iframe.mozRequestFullScreen();
        } else if (iframe.webkitRequestFullscreen) {
            iframe.webkitRequestFullscreen(Element.ALLOW_KEYBOARD_INPUT);
        }
    };
}

customElements.define("wasm-frame", WasmFrame);
