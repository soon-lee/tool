import {JSXElement} from "solid-js";

interface MainLayout{
    menu:JSXElement;
    view:JSXElement;
    todo:JSXElement;
    form:JSXElement;
}
export const MainLayout = (props:MainLayout) =>{
    return (
        <div class="layout">
            <div class="menu">{props.menu}</div>
            <div class="view">{props.view}</div>
            <div class="todo">{props.todo}</div>
            <div class="form">{props.form}</div>
        </div>
    )
}