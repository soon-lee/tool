import './App.css'
import {MainLayout} from "./containers/MainLayout.tsx";
import {Menu} from "./components/Menu.tsx";
import {View} from "./components/View.tsx";
import {Todo} from "./components/Todo.tsx";
import {Form} from "./components/Form.tsx";

function App () {

    return (
        <MainLayout
            menu={<Menu menus={[{name: '1'}]}/>}
            view={<View />}
            todo={<Todo />}
            form={<Form />}/>
    )
}

export default App
