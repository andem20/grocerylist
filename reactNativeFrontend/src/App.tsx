import {NavigationContainer} from '@react-navigation/native';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {LoginScreen} from './screens/LoginScreen';
import {ListScreen} from './screens/ListScreen';
import {useSelector} from 'react-redux';
import {UserStoreState} from './stores/UserStore';

export default function App() {
  const Stack = createNativeStackNavigator();

  const userToken = useSelector<UserStoreState, string | null>(
    state => state.userToken,
  );

  console.log(userToken);

  return (
    <NavigationContainer>
      <Stack.Navigator>
        {userToken ? (
          <>
            <Stack.Screen
              name="Lists"
              component={ListScreen}
              options={{title: 'Lists'}}
            />
          </>
        ) : (
          <>
            <Stack.Screen
              name="Login"
              component={LoginScreen}
              options={{title: 'Login'}}
            />
          </>
        )}
      </Stack.Navigator>
    </NavigationContainer>
  );
}
