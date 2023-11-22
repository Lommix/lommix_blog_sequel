class BevyRunner extends HTMLElement {
	constructor() {}
	connectedCallback() {
		console.log("MyComponent added to the page.");
	}
	disconnectedCallback() {
		console.log("MyComponent removed from the page.");
	}
	attributeChangedCallback(name, oldValue, newValue) {
		console.log(`Attribute: ${name} changed to ${newValue}`);
	}
	// Specify observed attributes so attributeChangedCallback will work
	static get observedAttributes() {
		return ["some-attribute"];
	}
}
export default BevyRunner;
