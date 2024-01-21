import type { GeneratorOptions } from "./types";

export default function generatePassword(length: number, options: GeneratorOptions): string {
    const lowercase = "abcdefghijklmnopqrstuvwxyz";
    const uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const numbers = "0123456789";
    const symbols = "!@#$%^&*()_+-=[]{};':\"\\|,.<>/?`~";
    let password = "";
    let characterSet = "";

    if (options.minLowercase > 0) {
        for (let i = 0; i < options.minLowercase; i++) {
            password += lowercase[Math.floor(Math.random() * lowercase.length)];
        }
        characterSet += lowercase;
    }

    if (options.minUppercase > 0) {
        for (let i = 0; i < options.minUppercase; i++) {
            password += uppercase[Math.floor(Math.random() * uppercase.length)];
        }
        characterSet += uppercase;
    }

    if (options.minNumbers > 0) {
        for (let i = 0; i < options.minNumbers; i++) {
            password += numbers[Math.floor(Math.random() * numbers.length)];
        }
        characterSet += numbers;
    }

    if (options.minSymbols > 0) {
        for (let i = 0; i < options.minSymbols; i++) {
            password += symbols[Math.floor(Math.random() * symbols.length)];
        }
        characterSet += symbols;
    }

    for (let i = 0; i < length - password.length; i++) {
        password += characterSet[Math.floor(Math.random() * characterSet.length)];
    }

    return password.split("").sort(() => Math.random() - 0.5).join("");
}