export function parseURL(url: string) {
  if (!url) return "";
  if (url.length === 0) return "";
  if (!url.startsWith("http://") && !url.startsWith("https://")) {
    return "https://" + url;
  }
  return url;
}

export function getDomain(url: string) {
  return new URL(url).hostname;
}

export function updateTheme(theme: string) {
  localStorage.setItem("theme", theme);
  document.documentElement.setAttribute("data-theme", theme);
}

// TODO: tweak criteria for password strength
/**
@param password - The password to check for strength
@return The strength of the password as a number between 0 and
*/
export function passwordStrength(password: string): number {
  let score = 0;

  const lengthCriteria = password.length >= 8;
  const upperCaseCriteria = /[A-Z]/.test(password);
  const lowerCaseCriteria = /[a-z]/.test(password);
  const numberCriteria = /[0-9]/.test(password);
  const specialCharCriteria = /[!@#$%^&*(),.?":{}|<>]/.test(password);

  if (lengthCriteria) score += 1;
  if (upperCaseCriteria) score += 1;
  if (lowerCaseCriteria) score += 1;
  if (numberCriteria) score += 1;
  if (specialCharCriteria) score += 1;

  return score;
}
