class SiteSearch extends HTMLElement{
    constructor() {
        super();
        this.attachShadow({mode:"open"});
    }
    async place(){
        const style = `
            :host{
                width: max(300px, 50%);
                padding: 5px;
                display: inline-flex;
                justify-content: space-between;
                align-items: center;
                gap: 5px;
                border: 1px solid #3D3D3D;
                border-radius: 5px;
                color: var(--text-color-primary);
            }
            :host(:focus-within){
                border-color: var(--color-info);
            }
            svg.icon{
                cursor: pointer;
                width: 25px;
                height: 25px;
            }
            input[type="text"]{
                border: none;
                background-color: transparent;
                margin: 0;
                font-family: inherit;
                font-size: inherit;
                font-weight: inherit;
                line-height: inherit;
                outline: none;
                box-shadow: none;
                appearance: none;
                color: inherit;
                width: calc(100% - 50px);
                height: 30px;
                padding: 0 5px 0 10px;
            }
        
            input[type="text"]:focus{
                box-shadow: none;
                outline: none;
            }
            input[type="text"]::placeholder{
                opacity: 0.5;
            }
        `;
        this.shadowRoot.append(createFragment([
            createStyle(style),
            await loadSvg("svg/search/search.svg"),
            createInput("text","please input "),
            await loadSvg("svg/search/cancel.svg")
        ]))
    }
    connectedCallback(){
        this.place().then();
    }
}
customElements.define("site-search", SiteSearch);