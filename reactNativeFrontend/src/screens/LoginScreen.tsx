import React, {useEffect, useState} from 'react';
import {Button, TextInput, View} from 'react-native';
import {setUserToken, store} from '../stores/UserStore';
import {useDispatch} from 'react-redux';
import {connectWebSocket} from '../middleware/ws.middleware';

export function LoginScreen() {
	const dispatch = useDispatch();

	const [credentials, setCredentials] = useState({
		username: 'anderslm@hotmail.com',
		password: 'password',
	});

	return (
		<View>
			<TextInput
				placeholder="Username"
				onChangeText={value =>
					setCredentials({
						username: value,
						password: credentials.password,
					})
				}
			/>
			<TextInput
				placeholder="Password"
				secureTextEntry={true}
				onChangeText={value =>
					setCredentials({
						username: credentials.username,
						password: value,
					})
				}
			/>
			<Button
				title="Login"
				onPress={() => {
					fetch(`http://${store.getState().backendUrl}/auth/login`, {
						method: 'POST',
						headers: {
							'content-type': 'application/json',
						},
						body: JSON.stringify(credentials),
					})
						.then(async response => {
							if (response.status === 200) {
								const token = await response.text();
								dispatch(setUserToken(token));
								store.dispatch(
									connectWebSocket(
										`ws://192.168.123.31:8080/ws?token=${
											store.getState().userToken
										}`,
									),
								);
							}
						})
						.catch(error => {
							console.error(error);
						});
				}}
			/>
		</View>
	);
}
