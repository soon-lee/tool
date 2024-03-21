import {For} from "solid-js";
import viteLogo from "../assets/vite.svg";
import solidLogo from "../assets/solid.svg";
interface MenuItem{
    name:string;
    children?:MenuItem;
}
interface MenuProps{
    menus: MenuItem[]
}
export const Menu = (props:MenuProps)=>{
    return (
        <div>
            <ul>
                <For each={props.menus}>
                    {item => (<li>{item.name}</li>)}
                </For>
            </ul>
            <div>
                <a href="https://vitejs.dev" target="_blank">
                    <img src={viteLogo} class="logo" alt="Vite logo"/>
                </a>
                <a href="https://solidjs.com" target="_blank">
                    <img src={solidLogo} class="logo solid" alt="Solid logo"/>
                </a>
            </div>
        </div>
    )
}