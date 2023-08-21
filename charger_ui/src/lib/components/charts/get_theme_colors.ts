/** Get currently active theme colors */

export class ThemeColors {

    constructor() { }
    primary_color: string = "red";
    primary_focus_color: string = "red";
    secondary_color: string = "red";
    secondary_focus_color: string = "red";
    accent_color: string = "red";
    accent_focus_color: string = "red";
    base_content: string = "red";

    static get_current_colors(): ThemeColors {
        let colors = new ThemeColors();
        const root = document.querySelector(':root');
        if (root) {
            colors.primary_color = getComputedStyle(root).getPropertyValue('--p');
            colors.primary_focus_color = getComputedStyle(root).getPropertyValue('--pf');
            colors.secondary_color = getComputedStyle(root).getPropertyValue('--s');
            colors.secondary_focus_color = getComputedStyle(root).getPropertyValue('--sf');
            colors.accent_color = getComputedStyle(root).getPropertyValue('--a');
            colors.accent_focus_color = getComputedStyle(root).getPropertyValue('--af');
            colors.base_content = getComputedStyle(root).getPropertyValue('--bc');
        }

        return colors;
    }
}
