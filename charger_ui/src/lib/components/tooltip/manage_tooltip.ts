// Logic to position and show/hide tooltips
import { computePosition, flip, shift, offset, arrow } from '@floating-ui/dom';

/**
 * Updates the position and visibility of a tooltip.
 * The tooltip is flipped if its original position would force it outside the viewport.
 */
export async function update_tooltip(tooltip: HTMLElement, anchor: HTMLElement, tooltip_arrow: HTMLElement) {
    if (!(tooltip && anchor && tooltip_arrow)) {
        return;
    }

    let pos = await computePosition(anchor, tooltip, {
        placement: 'bottom',
        middleware: [offset(6), flip(), shift({ padding: 5 }), arrow({ element: tooltip_arrow })]
    });
    tooltip.style.left = `${pos.x}px`;
    tooltip.style.top = `${pos.y}px`;
    const arrowX = pos.middlewareData.arrow?.x;
    const arrowY = pos.middlewareData.arrow?.y;

    const staticSide: any = {
        top: 'bottom',
        right: 'left',
        bottom: 'top',
        left: 'right'
    }[pos.placement.split('-')[0]];
    Object.assign(tooltip_arrow.style, {
        left: arrowX != null ? `${arrowX}px` : '',
        top: arrowY != null ? `${arrowY}px` : '',
        right: '',
        bottom: '',
        [staticSide]: '-4px'
    });
}

export function show_tooltip(tooltip: HTMLElement, anchor: HTMLElement, tooltip_arrow: HTMLElement) {
    tooltip.style.display = 'block';
    update_tooltip(tooltip, anchor, tooltip_arrow);
}

export function hide_tooltip(tooltip: HTMLElement) {
    tooltip.style.display = '';
}

/**
 * Register events to handle showing/hiding the tooltip on hover on mobile and desktop.
 * @param tooltip
 * @param anchor
 * @param tooltip_arrow
 */
export function register_events(tooltip: HTMLElement, anchor: HTMLElement, tooltip_arrow: HTMLElement) {
    if (!anchor) {
        return;
    }
    anchor.addEventListener('mouseenter', () => show_tooltip(tooltip, anchor, tooltip_arrow));
    anchor.addEventListener('mouseleave', () => hide_tooltip(tooltip));
    anchor.addEventListener('focus', () => show_tooltip(tooltip, anchor, tooltip_arrow));
    anchor.addEventListener('blur', () => hide_tooltip(tooltip));
}

/**
 * De-register events to handle showing/hiding the tooltip on hover on mobile and desktop.
 * @param tooltip
 * @param anchor
 * @param tooltip_arrow
 * @returns
 */
export function de_register_events(tooltip: HTMLElement, anchor: HTMLElement, tooltip_arrow: HTMLElement) {
    if (!anchor) {
        return;
    }
    anchor.removeEventListener('mouseenter', () => show_tooltip(tooltip, anchor, tooltip_arrow));
    anchor.removeEventListener('mouseleave', () => hide_tooltip(tooltip));
    anchor.removeEventListener('focus', () => show_tooltip(tooltip, anchor, tooltip_arrow));
    anchor.removeEventListener('blur', () => hide_tooltip(tooltip));
}
