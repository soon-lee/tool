class SiteTheme extends HTMLElement {
    constructor(props) {
        super();
        this.attachShadow({mode: "open"});
    }

    async load() {
        this.uxLightThemeIcon = await loadSvg("svg/theme/light.svg");
        this.uxDarkThemeIcon = await loadSvg("svg/theme/dark.svg");
    }

    place() {
        const style = `
            :host{
               pointer-events: all;
               cursor: pointer; 
            }
            svg.icon{
                width: 25px;
                height: 25px;
            }
        `;
        this.shadowRoot.append(createFragment([
            createStyle(style),
            this.dataset["theme"] === "light" ? this.uxLightThemeIcon : this.uxDarkThemeIcon,
        ]));
    }

    action() {
        const theme = this.shadowRoot.querySelector("svg.icon");
        this.uxLightThemeIcon.addEventListener("click", () => {
            this.shadowRoot.replaceChild(this.uxDarkThemeIcon, this.uxLightThemeIcon);
            this.dataset["theme"] = "dark";
            changeTheme("dark");
        });
        this.uxDarkThemeIcon.addEventListener("click", () => {
            this.shadowRoot.replaceChild(this.uxLightThemeIcon, this.uxDarkThemeIcon);
            this.dataset["theme"] = "light";
            changeTheme("light");
        });
    }

    connectedCallback() {
        !this.dataset["theme"] && (this.dataset["theme"] = loadTheme());
        this.load().then(() => {
            this.place();
            this.action();
        });
    }
}

customElements.define("site-theme", SiteTheme);