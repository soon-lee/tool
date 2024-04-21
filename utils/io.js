const loadSvg = async url => {
    const response = await fetch(url);
    if (!response.ok) throw new Error(`Failed to load SVG: ${url}`);

    const svgText = await response.text();
    const parser = new DOMParser();
    const svgDoc = parser.parseFromString(svgText, 'image/svg+xml');

    return svgDoc.documentElement;
}

const validateLocale = locale=>{
    if(locale){
        const matched = locale.match(/^([a-zA-Z]{2})[-_]?([a-zA-Z]{2})?$/);
        if(matched){
            locale = `${matched[1].toLowerCase()}`;
            matched[2] && (locale = `${locale}-${matched[2].toUpperCase()}`);
        }
    }
    return locale;
}
const changeLocale = locale => {
    let validLocale = validateLocale(locale);
    if (!validLocale) {
        throw new Error("Bad Locale Identifier!");
    }
    document.documentElement.setAttribute("lang", validLocale);
    localStorage.setItem("ux-nsp-locale", validLocale);
    return validLocale;
}
const loadLocale = () => {
    let locale = localStorage.getItem("ux-nsp-locale");
    if (!locale) {
        locale = changeLocale(navigator.language || "zh-CN");
    }
    return locale;
}
const changeTheme = theme => {
    const cacheTheme = localStorage.getItem("ux-nsp-theme");
    if(cacheTheme !== theme){
        document.documentElement.setAttribute("data-theme", theme);
        localStorage.setItem("ux-nsp-theme", theme);
    }
}
const loadTheme = () => {
    let theme = localStorage.getItem("ux-nsp-theme");
    if(!theme){
        theme = "light";
    }
    return theme;
}