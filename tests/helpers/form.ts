export const form = (data: Record<string, string>) => {
    return new URLSearchParams(data).toString();
}
