import React, {useState} from 'react';
import {
	LayoutAnimation,
	Platform,
	StyleSheet,
	Text,
	TouchableOpacity,
	UIManager,
	View,
} from 'react-native';

export const AnimationTest = () => {
	const [expanded, setExpanded] = useState(false);
	const [items, setItems] = useState([
		{name: 4},
		{name: 3},
		{name: 2},
		{name: 1},
	]);

	if (
		Platform.OS === 'android' &&
		UIManager.setLayoutAnimationEnabledExperimental
	) {
		UIManager.setLayoutAnimationEnabledExperimental(true);
	}

	return (
		<View style={style.container}>
			{items.map(item => (
				<View style={style.item} key={item.name}>
					<Text
						style={{
							fontSize: 20,
							alignSelf: 'center',
							padding: 10,
						}}>
						{item.name}
					</Text>
				</View>
			))}
			<TouchableOpacity
				onPress={() => {
					LayoutAnimation.configureNext(
						LayoutAnimation.Presets.spring,
					);

					setItems([...items.reverse()]);
				}}>
				<Text>Press me to!</Text>
			</TouchableOpacity>
			{/* <TouchableOpacity
				onPress={() => {
					LayoutAnimation.configureNext(
						LayoutAnimation.Presets.spring,
					);
					setExpanded(!expanded);
				}}>
				<Text>Press me to {expanded ? 'collapse' : 'expand'}!</Text>
			</TouchableOpacity>
			{expanded && (
				<View style={style.tile}>
					<Text>I disappear sometimes!</Text>
				</View>
			)} */}
		</View>
	);
};

const style = StyleSheet.create({
	tile: {
		backgroundColor: 'lightgrey',
		borderWidth: 0.5,
		borderColor: '#d6d7da',
	},
	container: {
		flex: 1,
		flexDirection: 'row',
		flexWrap: 'wrap',
		justifyContent: 'center',
		alignItems: 'center',
		overflow: 'hidden',
	},
	item: {
		width: '40%',
		backgroundColor: '#ffffff',
		margin: 5,
	},
});
