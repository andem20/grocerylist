export const COLORS = [
	'#264653',
	'#2A9D8F',
	'#E9C46A',
	'#F4A261',
	'#E76F51',
	'#4B0082',
	'#8B00FF',
];

export function getRandomColor() {
	return COLORS[Math.floor(Math.random() * COLORS.length)];
}
