import {View, StyleSheet, ScrollView} from 'react-native';
import React, {useEffect, useState} from 'react';
import {Item, UserStoreState, store, updateListItem} from '../stores/UserStore';
import {useSelector} from 'react-redux';
import {RootStackParamList} from '../App';
import {NativeStackScreenProps} from '@react-navigation/native-stack';
import {Input, Text} from '@rneui/base';
import {
	clearWatch,
	getDistanceFromLatLon,
	watchPosition,
} from '../services/LocationService';
import {GeoCoordinates} from 'react-native-geolocation-service';
import {RenderItem} from '../components/items/RenderItem';

type Props = NativeStackScreenProps<RootStackParamList, 'Items'>;

export function ItemsScreen({route}: Props) {
	const list = route.params.list;

	const [itemInput, setItemInput] = useState('');
	const [location, setLocation] = useState<GeoCoordinates>();
	const [itemList, setItemList] = useState<Item[]>([]);

	let numCategories: {
		[key: number]: number;
	} = {};

	const items = useSelector<UserStoreState, {[key: string]: Item}>(state => {
		const items = state.lists[list.id].items;
		numCategories = {};

		Object.values(items)
			.filter(item => !item.done)
			.forEach(item => {
				const category = numCategories[item.category ?? 0];
				numCategories[item.category ?? 0] =
					category == undefined || null ? 1 : category + 1;
			});

		return items;
	});

	useEffect(() => {
		setItemList(
			Object.values(items).sort((a, b) => {
				const distA = getDistanceFromLatLon(
					location?.latitude ?? 0,
					location?.longitude ?? 0,
					a.lat ?? 0,
					a.lng ?? 0,
				);

				const distB = getDistanceFromLatLon(
					location?.latitude ?? 0,
					location?.longitude ?? 0,
					b.lat ?? 0,
					b.lng ?? 0,
				);

				return distA - distB;
			}),
		);
	}, [items, location]);

	useEffect(() => {
		let watchId: number;

		watchPosition(position => {
			setLocation(position);
		}).then(id => (watchId = id!));

		return () => clearWatch(watchId);
	}, []);

	useEffect(() => {
		fetch(`http://${store.getState().backendUrl}/list/${list.id}/items`, {
			headers: {
				Authorization: store.getState().userToken ?? '',
			},
		})
			.then(response => response.json())
			.then((items: Item[]) => {
				items.forEach(item => store.dispatch(updateListItem(item)));
			})
			.catch(error => console.error(error));
	}, []);

	return (
		<ScrollView style={{backgroundColor: '#ffffff'}}>
			<View
				style={{
					flexDirection: 'row',
					flexWrap: 'wrap',
				}}>
				{itemList
					.sort((a, b) => (a.done === b.done ? 0 : a.done ? 1 : -1))
					.map(item => (
						<RenderItem
							item={item}
							numCategories={numCategories}
							location={location}
							key={item.id}
						/>
					))}
				<View style={[styles.itemInputContainer]}>
					{/* <Input
						placeholder="Enter item"
						leftIcon={{
							type: 'material-community',
							name: 'playlist-plus',
						}}
						value={itemInput}
						onEndEditing={() => {
							fetch(
								`http://${store.getState().backendUrl}/list/${
									list.id
								}/items`,
								{
									headers: {
										Authorization:
											store.getState().userToken ?? '',
										'Content-Type': 'application/json',
									},
									method: 'POST',
									body: JSON.stringify({
										name: itemInput,
									}),
								},
							),
								setItemInput('');
						}}
						onChangeText={value => setItemInput(value)}
					/> */}
					<Text>
						{location?.latitude}, {location?.longitude}
					</Text>
				</View>
			</View>
		</ScrollView>
	);
}

const styles = StyleSheet.create({
	container: {
		flex: 1,
	},
	listRow: {
		flexDirection: 'column',
	},
	itemInput: {
		flex: 1,
		fontSize: 20,
		padding: 20,
		backgroundColor: '#ffffff',
	},
	itemInputContainer: {
		flexDirection: 'row',
		marginTop: 10,
		marginBottom: 20,
	},
});
