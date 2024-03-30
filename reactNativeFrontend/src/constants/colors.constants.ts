// export const COLORS = [
// 	'#264653',
// 	'#2A9D8F',
// 	'#E9C46A',
// 	'#F4A261',
// 	'#E76F51',
// 	'#4B0082',
// 	'#8B00FF',
// ];

export const COLORS = [
	'#2E96D3',
	'#F75C03',
	'#F1C40F',
	'#BC4DEB',
	'#D90368',
	'#00CC66',
];

export function getRandomColor() {
	return COLORS[Math.floor(Math.random() * COLORS.length)];
}
