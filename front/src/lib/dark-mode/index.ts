/**
 * Toggle dark mode
 */
export function toggle() {
    const classes = document.documentElement.classList;

    if (classes.contains("dark")) {
        classes.remove("dark");
        return false;
    }

    classes.add("dark");
    return true;
}

export function isDark(): boolean {
    return document.documentElement.classList.contains("dark");
}
