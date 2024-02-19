import { AppRegistry } from 'react-native';
import { name as appName } from './app.json';
import App from './src/App';
import {store} from './src/stores/UserStore';
import { Provider } from 'react-redux';

function Main() {
    return (
        <Provider store={store}>
            <App />
        </Provider>
    )
}

AppRegistry.registerComponent(appName, () => Main);