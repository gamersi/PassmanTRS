export function parseURL(url: string) {
    if (!url) return ''
    if (url.length === 0) return ''
    if (!url.startsWith('http://') && !url.startsWith('https://')) {
        return 'https://' + url
    }
    return url
}

export function getDomain(url: string) {
    return new URL(url).hostname
}