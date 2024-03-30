import {Animated, Pressable, StyleSheet, View} from 'react-native';
import {Button, Icon, ScreenWidth, ListItem} from '@rneui/base';
import React, {useEffect, useRef} from 'react';
import {Item, store} from '../../stores/UserStore';
import {COLORS} from '../../constants/colors.constants';
import {getDistanceFromLatLon} from '../../services/LocationService';
import {GeoCoordinates} from 'react-native-geolocation-service';
import {CATEGORY_ICONS} from '../../constants/categories.constants';

interface Props {
	item: Item;
	numCategories: number;
	location: GeoCoordinates | undefined;
}

export function RenderItem({item, numCategories, location}: Props) {
	const margin = 3;
	const animatedWidth = useRef(
		new Animated.Value(
			ScreenWidth / (numCategories > 1 ? 2 : 1) - margin * 2,
		),
	).current;

	useEffect(() => {
		animatedWidth.setValue(
			ScreenWidth / (numCategories > 1 ? 2 : 1) - margin * 2,
		);
	}, [numCategories]);

	return (
		<Animated.View
			style={[
				styles.listRow,
				{
					width: animatedWidth,
					margin,
					overflow: 'hidden',
					elevation: 2,
					backgroundColor: '#ffffff',
					shadowColor: '#000000ff',
					shadowOffset: {
						width: 2,
						height: 2,
					},
					shadowOpacity: 1,
					shadowRadius: 1,
					borderRadius: 5,
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
						// borderWidth: 1,
						borderRadius: 5,
						opacity: item.done ? 0.6 : 1.0,
						backgroundColor: item.done
							? '#bbbbbb'
							: COLORS[item.category ?? 0] + '00',
						borderColor: COLORS[item.category ?? 0],
						justifyContent: 'center',
						alignItems: 'center',
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
							{getDistanceFromLatLon(
								location?.latitude ?? 0,
								location?.longitude ?? 0,
								item.lat ?? 0,
								item.lng ?? 0,
							).toFixed(1)}
							m
						</ListItem.Subtitle>
					</ListItem.Content>
					<Icon
						name={(() =>
							Object.keys(CATEGORY_ICONS)
								.filter(k =>
									k.includes(item.name.toLowerCase()),
								)
								.map(k => {
									return CATEGORY_ICONS[k];
								})[0])()}
						type="material-community"
						color="#ffffff"
						size={22}
						style={{
							padding: 10,
							margin: 5,
							justifyContent: 'center',
							borderRadius: 100,
							backgroundColor: COLORS[item.category ?? 0],
						}}
					/>
				</Pressable>
			</ListItem.Swipeable>
		</Animated.View>
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
