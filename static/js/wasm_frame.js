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
			@keyframes puls {
				0% { transform: blur(0.5rem); }
				50% { transform: blur(0.8rem); }
				100% { transform: blur(0.5rem); }
			}
			</style>
			<div class="wrapper">
				<iframe id="wasm-frame" src=""></iframe>
				<div id="wasm-loader">
					<img src="" alt="WasmFrame" id="wasm-cover">
					<a class="centered-abs" id="load-button">Load & Play</a>
				</div>
			</div>
		`;
    }
    connectedCallback() {
        this.shadowRoot.getElementById("load-button").onclick = () => this.play();
        this.shadowRoot.getElementById("wasm-cover").src = this.getAttribute("cover");
    }

    play = () => {
        this.shadowRoot.getElementById("wasm-loader").style.display = "none";
        this.shadowRoot.getElementById("wasm-frame").style.display = "block";
        const wasmFrame = this.shadowRoot.getElementById("wasm-frame");
        wasmFrame.src = this.getAttribute("src");
    };
}

customElements.define("wasm-frame", WasmFrame);
