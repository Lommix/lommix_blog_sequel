class BevyRunner extends HTMLElement {
	constructor() {
		super();

		// base style
		this.style.margin = "3rem auto";
		this.style.display = "block";
		this.style.position = "relative";

		// canvas
		this.canvas = document.createElement("canvas");
		this.canvas.id = this.getAttribute("canvas-id");
		this.canvas.height = this.getAttribute("height");
		this.canvas.style.width = "100%";
		this.canvas.style.display = "none";
		this.canvas.oncontextmenu = (e) => e.preventDefault();
		this.appendChild(this.canvas);

		// loading screen
		this.loadingScreen = document.createElement("div");
		this.loadingScreen.id = "loading-screen";
		this.loadingScreen.style.height = `${this.getAttribute("height")}px`;
		this.loadingScreen.style.backgroundColor = "#000000";
		this.loadingScreen.style.display = "none";
		this.loadingScreen.innerHTML = `
			<style>
				.spinner-container{
					display: flex;
					align-items: center;
					justify-content: center;
					color: #FFF;
					height: 100%;
					font-size: 2rem;
				}
				.spinner{
				  animation: spin 1s linear infinite;
				}

				@keyframes spin {
				  0% { transform: rotate(0deg); }
				  100% { transform: rotate(360deg); }
				}
				.spinner {
					margin-right: 1rem;
					border-top-color: #FFF;
					border-left-color: #FFF;
					animation: spin 1s linear infinite;
					border-radius: 50%;
					border-width: 5px;
					border-style: solid;
					border-right-color: transparent;
					border-bottom-color: transparent;
					width: 34px;
					height: 34px;
				}/*}}}*/

			</style>
            <div class="spinner-container">
                <div class="spinner"></div>Loading
            </div>
        `;

		this.appendChild(this.loadingScreen);
		// add button to run load
		this.loadButton = document.createElement("button");
		this.loadButton.style = `
			position: absolute;
			display: block;
			margin: 0 auto;
	 		top: 50%;
			left: 50%;
			transform: translate(-50%, -50%);
			padding: 1rem 2rem;
			font-size: 1.5rem;
			font-weight: 600;
			color: #FFFFFF;
			border: none;
			border-radius: 3px;
		`;

		this.loadButton.innerHTML = "Load & Play";
		this.loadButton.onclick = () => this.load();
		this.appendChild(this.loadButton);
	}

	async load() {
		const wasm_path = this.getAttribute("wasm-path");
		const script_path = this.getAttribute("script-path");
		const canvas_id = this.getAttribute("canvas-id");
		const script = await import(script_path);

		//delete all image children
		this.querySelectorAll("img").forEach((img) => img.remove());
		this.loadButton.style.display = "none";
		this.loadingScreen.style.display = "block";
		this.loadingScreen.scrollIntoView({
			behavior: "smooth",
		});

		fetch(wasm_path)
			.then((res) => res.arrayBuffer())
			.then(async (compressedBytes) => {
				const bytes = pako.inflate(new Uint8Array(compressedBytes));
				await script.initSync(bytes);
				await script.init();
				this.loadingScreen.style.display = "none";
				this.canvas.style.display = "block";
				await script.run(
					"#" + canvas_id,
					this.canvas.clientWidth,
					this.canvas.height,
				);
			});
	}
}

customElements.define("bevy-runner", BevyRunner);
export default BevyRunner;
