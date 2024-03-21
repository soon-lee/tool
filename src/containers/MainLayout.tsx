import {JSXElement} from "solid-js";

interface MainLayout{
    menu:JSXElement;
    view:JSXElement;
    todo:JSXElement;
    form:JSXElement;
}
export const MainLayout = (props:MainLayout) =>{
    return (
        <div class="od-layout">
            <div class="od-menu">{props.menu}</div>
            <div class="od-view">{props.view}</div>
            <div class="od-todo">{props.todo}</div>
            <div class="od-form">{props.form}</div>
        </div>
    )
}