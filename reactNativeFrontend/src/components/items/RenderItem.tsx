import {Pressable, StyleSheet, View} from 'react-native';
import {Button, Icon, ScreenWidth, ListItem} from '@rneui/base';
import React from 'react';
import {Item, store} from '../../stores/UserStore';
import {COLORS} from '../../constants/colors.constants';
import {getDistanceFromLatLon} from '../../services/LocationService';
import {GeoCoordinates} from 'react-native-geolocation-service';

interface Props {
	item: Item;
	numCategories: {[key: number]: number};
	location: GeoCoordinates | undefined;
}

export function RenderItem({item, numCategories, location}: Props) {
	const margin = 3;
	return (
		<View
			style={[
				styles.listRow,
				{
					width:
						ScreenWidth /
							(numCategories[item.category ?? 0] > 1 ? 2 : 1) -
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
								`http://${store.getState().backendUrl}/list/${
									item.list_id
								}/items`,
								{
									headers: {
										Authorization:
											store.getState().userToken ?? '',
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
							{item.category} |{' '}
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
