import {
	View,
	StyleSheet,
	Pressable,
	ScrollView,
	PermissionsAndroid,
} from 'react-native';
import React, {useEffect, useState} from 'react';
import {Item, UserStoreState, store, updateListItem} from '../stores/UserStore';
import {useSelector} from 'react-redux';
import {RootStackParamList} from '../App';
import {NativeStackScreenProps} from '@react-navigation/native-stack';
import {Button, Icon, Input, ListItem, ScreenWidth, Text} from '@rneui/base';
import {COLORS} from '../constants/colors.constants';
import Geolocation from 'react-native-geolocation-service';

type Props = NativeStackScreenProps<RootStackParamList, 'Items'>;

export function ItemsScreen({route}: Props) {
	const list = route.params.list;

	const [itemInput, setItemInput] = useState('');
	const [location, setLocation] = useState<Geolocation.GeoCoordinates>();
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
	}, [location, items]);

	useEffect(() => {
		let watch: number;
		requestLocationPermission().then(res => {
			if (res) {
				watch = Geolocation.watchPosition(
					position => {
						setLocation(position.coords);
					},
					error => {
						// See error code charts below.
						console.log(error.code, error.message);
					},
					{
						enableHighAccuracy: true,
						distanceFilter: 1,
						interval: 1000,
						forceRequestLocation: true,
						forceLocationManager: false,
						showLocationDialog: true,
						useSignificantChanges: false,
					},
				);
			}
		});

		return () => Geolocation.clearWatch(watch);
	}, []);

	function deg2rad(deg: number) {
		return deg * (Math.PI / 180);
	}

	function getDistanceFromLatLon(
		lat1: number,
		lon1: number,
		lat2: number,
		lon2: number,
	) {
		const earthRadiusMeters = 6371_000;

		const dLat = deg2rad(lat2 - lat1);
		const dLon = deg2rad(lon2 - lon1);

		const a =
			Math.sin(dLat / 2) * Math.sin(dLat / 2) +
			Math.cos(deg2rad(lat1)) *
				Math.cos(deg2rad(lat2)) *
				Math.sin(dLon / 2) *
				Math.sin(dLon / 2);

		const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
		const distance = earthRadiusMeters * c;

		return distance;
	}

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

	const RenderItem = (item: Item, numCategories: any) => {
		const margin = 3;

		return (
			<View
				style={[
					styles.listRow,
					{
						width:
							ScreenWidth /
								(numCategories[item.category ?? 0] > 1
									? 2
									: 1) -
							margin * 2,
						margin,
						overflow: 'hidden',
					},
				]}
				key={item.id}>
				<ListItem.Swipeable
					containerStyle={{
						backgroundColor: '#ffffff',
						padding: 0,
					}}
					leftContent={reset => (
						<Button
							title="Edit"
							onPress={() => {
								reset();
							}}
							icon={{name: 'edit', color: 'white'}}
							buttonStyle={{height: '100%', width: '100%'}}
						/>
					)}
					rightContent={reset => (
						<Button
							title="Delete"
							onPress={() => {
								reset();
								fetch(
									`http://${
										store.getState().backendUrl
									}/list/${item.list_id}/items`,
									{
										headers: {
											Authorization:
												store.getState().userToken ??
												'',
											'Content-Type': 'application/json',
										},
										method: 'DELETE',
										body: JSON.stringify({
											id: item.id,
										}),
									},
								);
							}}
							icon={{name: 'delete', color: 'white'}}
							buttonStyle={{
								height: '100%',
								backgroundColor: 'red',
								width: '100%',
							}}
						/>
					)}
					rightWidth={ScreenWidth / 4 + margin}
					leftWidth={ScreenWidth / 4 - margin * 4}>
					<Pressable
						onPress={async () => {
							fetch(
								`http://${store.getState().backendUrl}/list/${
									item.list_id
								}/items`,
								{
									headers: {
										Authorization:
											store.getState().userToken ?? '',
										'Content-Type': 'application/json',
									},
									method: 'PUT',
									body: JSON.stringify({
										id: item.id,
										name: item.name,
										done: !item.done,
										lat: item.lat,
										lng: item.lng,
										category: item.category,
									}),
								},
							);
						}}
						style={{
							flexDirection: 'row',
							borderWidth: 1,
							borderRadius: 5,
							opacity: item.done ? 0.6 : 1.0,
							backgroundColor: item.done
								? '#bbbbbb'
								: COLORS[item.category ?? 0] + '00',
							borderColor: COLORS[item.category ?? 0],
						}}>
						<ListItem.Content
							style={{
								padding: 10,
								margin: 10,
							}}>
							<ListItem.Title
								style={{
									fontSize: 18,
									fontWeight: 'bold',
									color: '#000000',
									textDecorationLine: item.done
										? 'line-through'
										: 'none',
								}}>
								{item.name}
							</ListItem.Title>
							<ListItem.Subtitle style={{color: '#000000'}}>
								{item.category} |
								{getDistanceFromLatLon(
									location?.latitude ?? 0,
									location?.longitude ?? 0,
									item.lat ?? 0,
									item.lng ?? 0,
								).toFixed(3)}
							</ListItem.Subtitle>
						</ListItem.Content>
						<Icon
							name="cart"
							type="material-community"
							color={COLORS[item.category ?? 0]}
							size={35}
							style={{
								flex: 1,
								padding: 5,
								justifyContent: 'center',
							}}
						/>
					</Pressable>
				</ListItem.Swipeable>
			</View>
		);
	};

	return (
		<ScrollView style={{backgroundColor: '#ffffff'}}>
			<View
				style={{
					flexDirection: 'row',
					flexWrap: 'wrap',
				}}>
				{itemList
					.sort((a, b) => (a.done === b.done ? 0 : a.done ? 1 : -1))
					.map(item => RenderItem(item, numCategories))}
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

const requestLocationPermission = async () => {
	try {
		const granted = await PermissionsAndroid.request(
			PermissionsAndroid.PERMISSIONS.ACCESS_FINE_LOCATION,
			{
				title: 'Geolocation Permission',
				message: 'Can we access your location?',
				buttonNeutral: 'Ask Me Later',
				buttonNegative: 'Cancel',
				buttonPositive: 'OK',
			},
		);
		if (granted === 'granted') {
			return true;
		} else {
			return false;
		}
	} catch (err) {
		return false;
	}
};
