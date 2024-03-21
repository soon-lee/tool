import {Application} from "pixi.js";
import {createEffect} from "solid-js";

export const View = ()=>{
    let container: HTMLDivElement | undefined;
    const application = new Application();

    createEffect(()=>{
        application.init({width:500,height:300,background:'lightgray'}).then(()=>{
            container?.appendChild(application.canvas)
        })
    })
    return (
        <div ref={container}></div>
    )
}