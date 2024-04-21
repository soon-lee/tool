class DoorLayout extends HTMLElement{
    constructor() {
        super();
        this.attachShadow({ mode: "open" });
    }
    place(){
        const style = `
            :host{
                display: grid;
                grid-template-columns: max(200px, calc(20% - 5px)) auto max(200px, calc(20% - 5px));
                grid-template-rows: 60px auto 40px;
                grid-template-areas: "header header header" "catalog content outline" "footer footer footer";
                grid-column-gap: 5px; 
            }
            div.header{
                grid-area: header;
                display: inline-flex;
                justify-content: space-between;
                align-items: center;
                gap: 5px;
                padding: 10px;
            }
            div.catalog{
                grid-area: catalog;
                background-color: #b00020;
            }
            div.content{
                grid-area: content;
                background-color: #6200ee;
            }
            div.outline{
                grid-area: outline;
                background-color: #018786;
            }
            div.footer{
                grid-area: footer;
                background-color: #d7bbff;
            }
        `;
        this.shadowRoot.append(createFragment([
            createStyle(style),
            createDivWithSlot("header"),
            createDivWithSlot("catalog"),
            createDivWithSlot("content"),
            createDivWithSlot("outline"),
            createDivWithSlot("footer")
        ]));
    }
    connectedCallback(){
        this.place();
    }
}
customElements.define("door-layout",DoorLayout);