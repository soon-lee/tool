const createFragment = children => {
    const fragment = document.createDocumentFragment();
    if (children) {
        children.forEach(child => {
            fragment.append(child);
        });
    }
    return fragment;
}
const createStyle = text => {
    const style = document.createElement("style");
    style.setAttribute("type", "text/css");
    style.textContent = text;
    return style;
}
const createDivWithSlot = name => {
    const div = document.createElement("div");
    const slot = document.createElement("slot");
    slot.setAttribute("name", name);
    div.setAttribute("class", name);
    div.append(slot);
    return div;
}
const createSpan = text =>{
    const span = document.createElement("span");
    span.textContent = text;
    return span;
}
const createInput = (type, placeholder) => {
    const input = document.createElement("input");
    input.setAttribute("type", type);
    placeholder && input.setAttribute("placeholder", placeholder);
    return input;
}