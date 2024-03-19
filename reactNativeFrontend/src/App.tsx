import {NavigationContainer} from '@react-navigation/native';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {LoginScreen} from './screens/LoginScreen';
import {ListsScreen} from './screens/ListsScreen';
import {ItemsScreen} from './screens/ItemsScreen';
import {useSelector} from 'react-redux';
import {List, UserStoreState} from './stores/UserStore';

export type RootStackParamList = {
	Lists: undefined;
	Items: {list: List};
	Login: undefined;
};

export default function App() {
	const Stack = createNativeStackNavigator<RootStackParamList>();

	const userToken = useSelector<UserStoreState, string | null>(
		state => state.userToken,
	);

	return (
		<NavigationContainer>
			<Stack.Navigator>
				{userToken ? (
					<>
						<Stack.Screen
							name="Lists"
							component={ListsScreen}
							options={{title: 'Lists'}}
						/>
						<Stack.Screen
							name="Items"
							component={ItemsScreen}
							options={({route}) => ({
								title: route.params.list.title,
							})}
						/>
					</>
				) : (
					<Stack.Screen
						name="Login"
						component={LoginScreen}
						options={{title: 'Login'}}
					/>
				)}
			</Stack.Navigator>
		</NavigationContainer>
	);
}
