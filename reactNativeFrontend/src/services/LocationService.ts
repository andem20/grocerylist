import Geolocation from 'react-native-geolocation-service';
import {PermissionsAndroid} from 'react-native';

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

function deg2rad(deg: number) {
	return deg * (Math.PI / 180);
}

export function getDistanceFromLatLon(
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

export async function watchPosition(
	callback: (position: Geolocation.GeoCoordinates) => void,
) {
	const hasPermission = await requestLocationPermission();

	if (hasPermission) {
		return Geolocation.watchPosition(
			position => {
				callback(position.coords);
			},
			error => {
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
}

export function clearWatch(watchId: number) {
	Geolocation.clearWatch(watchId);
}
