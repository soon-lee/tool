class SiteHome extends HTMLElement {
    constructor(props) {
        super();
        this.attachShadow({mode: "open"});
        this.addEventListener("newSiteHome", event => {
            this.dataset.program = event.detail["program"];
        });
    }

    async place() {
        const style = `
            :host{
                cursor: pointer;
                display: inline-flex;
                justify-content: space-around;
                align-items: center;
                color: var(--text-color-primary);
            }
            :host(:hover){
                opacity: 0.5;
            }
            svg{
                width: 50px;
                height: 50px;
            }
            span{
                white-space: nowrap;
                font-weight: bolder;
            }
        `;
        this.shadowRoot.append(createFragment([
            createStyle(style),
            await loadSvg(`svg/home/${this.dataset.program}.svg`),
            createSpan(`${this.dataset["program"]} Doc`)
        ]));
    }

    connectedCallback() {
        !this.dataset.program && (this.dataset.program = "javascript");
        this.place().then();
    }
}

customElements.define("site-home", SiteHome);