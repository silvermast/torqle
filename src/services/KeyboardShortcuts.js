import { register, unregister } from '@tauri-apps/plugin-global-shortcut';
// const register = () => null;
// const unregister = () => null;

/** @todo make these configurable */

class KeyCombo {
    keys = [];
    
    constructor(keys) {
        this.keys = keys;
    }

    /**
     * Formats the key binding into a tauri-compatible string
     * @returns String
     */
    forTauri() {
        return this.keys.join('+');
    }

    /**
     * Formats shortcut for Ace
     * @return Object ({ mac, win })
     * @example { mac: 'Command-Shift-D', win: 'Ctrl-Shift-D' }
     */
    forAce() {
        let keys = this.keys.join('-').replace(/Control/, 'Ctrl');
        return {
            win: keys,
            mac: keys.replace(/CommandOrCtrl/, 'Command'),
        }
    }
}

const global = {
    newWindow: new KeyCombo(['CommandOrControl', 'N']),
    newTab: new KeyCombo(['CommandOrControl', 'T']),
    
    closeWindow: new KeyCombo(['CommandOrControl', 'Shift', 'W']),
    closeTab: new KeyCombo(['CommandOrControl', 'W']),
    
    filterTables: new KeyCombo(['CommandOrControl', 'J']),
    selectDatabase: new KeyCombo(['CommandOrControl', 'K']),
};

const editor = {
    runQuery: new KeyCombo(['CommandOrControl', 'Enter']),
    addCursorAtNextItem: new KeyCombo(['CommandOrControl', 'D']),
    addCursorAtPrevItem: new KeyCombo(['CommandOrControl', 'Shift', 'D']),
    addCursorsAtSelectedLines: new KeyCombo(['Alt', 'Shift', 'I']),
    // Ctrl+Alt+Up: Add cursor above
    // Ctrl+Alt+Down: Add cursor below
};

export default { global, editor, register, unregister };