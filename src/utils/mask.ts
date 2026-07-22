export function encryptUsername(username: string): string {
	const length = username.length;
	const firstThirdLength = Math.round(length * 0.2);
	const firstPart = username.substring(0, firstThirdLength);
	const lastPart = username.substring(length - firstThirdLength);
	const middlePartLength = length - 2 * firstThirdLength;
	const middlePart = '*'.repeat(middlePartLength);
	return firstPart + middlePart + lastPart;
}

export function encryptPassword(password: string): string {
	return '*'.repeat(password.length);
}
