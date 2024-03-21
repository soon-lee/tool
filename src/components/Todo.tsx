import {createSignal, For, JSXElement} from "solid-js";

interface TabItem {
    name: string;
    content: JSXElement;
}

interface TodoProps {
    tabs: TabItem[];
    activeTab: number;
}

export const Todo = (props: TodoProps) => {

    const [activeTabId, setActiveTabId] = createSignal(props.activeTab)
    const onTabLabelClick = ( id: number) => {
        // console.log(event.target.classList)
        setActiveTabId(id);
    }

    return (
        <div class="tab-wrapper">
            <div class="tab-label">
                <For each={props.tabs}>{
                    (item, index) => <div
                        class={activeTabId() === index() ? "active" : ""}
                        onClick={() => onTabLabelClick(index())}>{item.name}</div>
                }</For>
            </div>
            <div class="tab-content">{props.tabs[activeTabId()].content}</div>
        </div>
    )
}