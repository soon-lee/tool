import './App.css'
import {MainLayout} from "./containers/MainLayout.tsx";
import {Menu} from "./components/Menu.tsx";
import {View} from "./components/View.tsx";
import {Todo} from "./components/Todo.tsx";
import {Form} from "./components/Form.tsx";

function App () {

    return (
        <MainLayout
            menu={<Menu menus={
                [{name: '1'},{name: 'sfs'},{name: '独立客观记录'}]
            }/>}
            view={<View />}
            todo={<Todo  activeTab={0} tabs={[{name:'1',content:'fjslkgdfjgl'},{name:'2',content:'2234236t46'}]}/>}
            form={<Form />}/>
    )
}

export default App
