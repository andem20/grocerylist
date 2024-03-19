import React from 'react';
import {View, Button, Pressable, StyleSheet, ScrollView} from 'react-native';
import {useSelector} from 'react-redux';
import {
	List,
	UserStoreState,
	clearLists,
	setUserToken,
	store,
} from '../stores/UserStore';
import {NativeStackScreenProps} from '@react-navigation/native-stack';
import {RootStackParamList} from '../App';
import {ListItem, Icon} from '@rneui/themed';
import {getRandomColor} from '../constants/colors.constants';

type Props = NativeStackScreenProps<RootStackParamList, 'Lists'>;

export function ListsScreen({navigation}: Props) {
	const lists = useSelector<UserStoreState, {[key: string]: List}>(
		state => state.lists,
	);

	return (
		<View
			style={[
				styles.container,
				{
					flexDirection: 'column',
				},
			]}>
			<ScrollView style={{flex: 1}}>
				{Object.values(lists).map((list, index) => (
					<Pressable
						key={list.id}
						onPress={() => {
							navigation.navigate('Items', {
								list,
							});
						}}
						style={[styles.listRow]}>
						<Icon
							name="cart"
							type="material-community"
							color="#ffffff"
							style={{
								backgroundColor: getRandomColor(),
								flex: 1,
								padding: 5,
								justifyContent: 'center',
							}}
						/>
						<ListItem key={index} style={{flex: 12}}>
							<ListItem.Content>
								<ListItem.Title
									style={{
										fontSize: 20,
										fontWeight: 'bold',
									}}>
									{list.title}
								</ListItem.Title>
								<ListItem.Subtitle>
									Last modified by some user
								</ListItem.Subtitle>
							</ListItem.Content>
						</ListItem>
					</Pressable>
				))}
			</ScrollView>
			<View style={{flex: 0}}>
				<Button
					title="Logout"
					onPress={() => {
						store.dispatch(setUserToken(null));
						store.dispatch(clearLists());
					}}
					color="#FF0000"
				/>
			</View>
		</View>
	);
}

const styles = StyleSheet.create({
	container: {
		flex: 1,
	},
	listRow: {
		flexDirection: 'row',
		backgroundColor: '#ffffff',
		margin: 5,
	},
});
