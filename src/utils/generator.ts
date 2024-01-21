import type { GeneratorOptions } from "./types";

export default function generatePassword(length: number, options: GeneratorOptions): string {
    const { minLowercase, minUppercase, minNumbers, minSymbols } = options;

    const lowercaseChars = 'abcdefghijklmnopqrstuvwxyz';
    const uppercaseChars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
    const numberChars = '0123456789';
    const symbolChars = '!@#$%^&*()-=_+[]{}|;:,.<>?';

    let password = '';

    for (let i = 0; i < minLowercase; i++) {
        password += lowercaseChars[Math.floor(Math.random() * lowercaseChars.length)];
    }

    for (let i = 0; i < minUppercase; i++) {
        password += uppercaseChars[Math.floor(Math.random() * uppercaseChars.length)];
    }

    for (let i = 0; i < minNumbers; i++) {
        password += numberChars[Math.floor(Math.random() * numberChars.length)];
    }

    for (let i = 0; i < minSymbols; i++) {
        password += symbolChars[Math.floor(Math.random() * symbolChars.length)];
    }

    const remainingChars = length - password.length;
    const allChars = lowercaseChars + uppercaseChars + numberChars + symbolChars;

    for (let i = 0; i < remainingChars; i++) {
        password += allChars[Math.floor(Math.random() * allChars.length)];
    }

    password = password.split('').sort(() => Math.random() - 0.5).join('');

    return password;
}