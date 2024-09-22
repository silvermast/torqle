import { Window, getCurrentWindow } from '@tauri-apps/api/window';
import { shortcuts } from './services/KeyboardShortcuts.js';
import { Menu, MenuItem, Submenu, PredefinedMenuItem } from '@tauri-apps/api/menu'
import { v4 as uuidv4 } from 'uuid';

/**
 * @warning this Menu API is not fully implemented in Tauri as of 2.0-RC. It is left here as a PoC and disabled.
 * 
 * @todo make this more extensible. Possibly pass in a Vue state to determine of "New Tab" is enabled/disabled based on the connection view or make it usable from the view itself.
 */

/**
 * Removes all children from a menu
 * @param {Menu|Submenu|MenuItem} menu 
 */
async function removeAllItems(menu) {
    try {
        const items = await menu.items();
        await Promise.all(items.map(item => menu.remove(item)));
    } catch (e) {
        console.error(e);
    }
}

/**
 * Initializes and customizes the app menu
 * @returns {Menu|null}
 */
async function initMenu() {
    try {
        shortcuts.newWindow.register(() => {
            new Window(uuidv4());
        });

        const menu = await Menu.default();

        let subMenus = await menu.items();
        for (const subMenu of subMenus) {
            const subMenuTitle = await subMenu.text();
            switch (subMenuTitle) {
                case 'File':
                    /**
                     * Tested on MacOS. Default "File" menu has only 1 item: "Close Window" which is mapped to Cmd+W.
                     * We can't override the shortcut, so we'll clear everything out and rebuild it.
                     */
                    await removeAllItems(subMenu);
                    const newMenuItems = await Promise.all([
                        MenuItem.new({
                            text: 'New Window',
                            accelerator: shortcuts.newWindow.formatForTauri(),
                            action() {
                                new Window(uuidv4());
                            },
                        }),
                        MenuItem.new({
                            text: 'New Tab',
                            accelerator: shortcuts.newTab.formatForTauri(),
                            // Action is defined and handled inside of ActionView.vue
                        }),
                        PredefinedMenuItem.new({ item: 'Separator' }),
                        MenuItem.new({
                            text: 'Close Tab',
                            accelerator: shortcuts.closeTab.formatForTauri(),
                            // Action is defined and handled inside of ActionView.vue
                        }),
                        MenuItem.new({
                            text: 'Close Window',
                            accelerator: shortcuts.closeWindow.formatForTauri(),
                            action() {
                                const window = getCurrentWindow();
                                window.close();
                            },
                        }),
                    ]);
                    await subMenu.append(newMenuItems);
                    break;
            }
        }

        window.menu = menu; // for debug
        return await menu.setAsAppMenu(); // this "synchronizes" the menu across all windows

    } catch (e) {
        console.error(e);
    }
}

export { initMenu };