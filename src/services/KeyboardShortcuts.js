import { register, unregister } from '@tauri-apps/plugin-global-shortcut';
// const register = () => null;
// const unregister = () => null;

/**
 * 3 types of shortcuts, based on context:
 * - Global
 * - Window
 * - Editor
 */

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
            win: keys.replace(/CommandOrCtrl/, 'Ctrl'),
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

    /* From command audit:
    Object.values(aceEditor.commands.commands).filter(c => !!c.bindKey).forEach(c => console.log(c.name, c.description, c.bindKey))
    */
    // scrollup                       --  Scroll up   --  Object { win: "Ctrl-Up", mac: null }
    // scrolldown                     --  Scroll down   --  Object { win: "Ctrl-Down", mac: null }
    // selectlinestart                --  Select line start Shift-Home
    // selectlineend                  --  Select line end Shift-End
    // passKeysToBrowser              --  Pass keys to browser   --  Object { win: null, mac: null }
    // replace                        --  Replace   --  Object { win: "Ctrl-H", mac: "Command-Option-F" }
    // undo                           --  Undo   --  Object { win: "Ctrl-Z", mac: "Command-Z" }
    // redo                           --  Redo   --  Object { win: "Ctrl-Shift-Z|Ctrl-Y", mac: "Command-Shift-Z|Command-Y" }
    // copylinesup                    --  Copy lines up   --  Object { win: "Alt-Shift-Up", mac: "Command-Option-Up" }
    // movelinesup                    --  Move lines up   --  Object { win: "Alt-Up", mac: "Option-Up" }
    // copylinesdown                  --  Copy lines down   --  Object { win: "Alt-Shift-Down", mac: "Command-Option-Down" }
    // movelinesdown                  --  Move lines down   --  Object { win: "Alt-Down", mac: "Option-Down" }
    // del                            --  Delete   --  Object { win: "Delete", mac: "Delete|Ctrl-D|Shift-Delete" }
    // backspace                      --  Backspace   --  Object { win: "Shift-Backspace|Backspace", mac: "Ctrl-Backspace|Shift-Backspace|Backspace|Ctrl-H" }
    // cut_or_delete                  --  Cut or delete   --  Object { win: "Shift-Delete", mac: null }
    // removetolinestart              --  Remove to line start   --  Object { win: "Alt-Backspace", mac: "Command-Backspace" }
    // removetolineend                --  Remove to line end   --  Object { win: "Alt-Delete", mac: "Ctrl-K|Command-Delete" }
    // removetolinestarthard          --  Remove to line start hard   --  Object { win: "Ctrl-Shift-Backspace", mac: null }
    // removetolineendhard            --  Remove to line end hard   --  Object { win: "Ctrl-Shift-Delete", mac: null }
    // removewordleft                 --  Remove word left   --  Object { win: "Ctrl-Backspace", mac: "Alt-Backspace|Ctrl-Alt-Backspace" }
    // removewordright                --  Remove word right   --  Object { win: "Ctrl-Delete", mac: "Alt-Delete" }
    // outdent                        --  Outdent   --  Object { win: "Shift-Tab", mac: "Shift-Tab" }
    // indent                         --  Indent   --  Object { win: "Tab", mac: "Tab" }
    // blockoutdent                   --  Block outdent   --  Object { win: "Ctrl-[", mac: "Ctrl-[" }
    // blockindent                    --  Block indent   --  Object { win: "Ctrl-]", mac: "Ctrl-]" }
    // touppercase                    --  To uppercase   --  Object { win: "Ctrl-U", mac: "Ctrl-U" }
    // tolowercase                    --  To lowercase   --  Object { win: "Ctrl-Shift-U", mac: "Ctrl-Shift-U" }
    // expandtoline                   --  Expand to line   --  Object { win: "Ctrl-Shift-L", mac: "Command-Shift-L" }
    // addCursorAbove                 --  Add cursor above   --  Object { win: "Ctrl-Alt-Up", mac: "Ctrl-Alt-Up" }
    // addCursorBelow                 --  Add cursor below   --  Object { win: "Ctrl-Alt-Down", mac: "Ctrl-Alt-Down" }
    // addCursorAboveSkipCurrent      --  Add cursor above (skip current)   --  Object { win: "Ctrl-Alt-Shift-Up", mac: "Ctrl-Alt-Shift-Up" }
    // addCursorBelowSkipCurrent      --  Add cursor below (skip current)   --  Object { win: "Ctrl-Alt-Shift-Down", mac: "Ctrl-Alt-Shift-Down" }
    // selectNextBefore               --  Select next before   --  Object { win: "Ctrl-Alt-Shift-Left", mac: "Ctrl-Alt-Shift-Left" }
    // selectNextAfter                --  Select next after   --  Object { win: "Ctrl-Alt-Shift-Right", mac: "Ctrl-Alt-Shift-Right" }
    // alignCursors                   --  Align cursors   --  Object { win: "Ctrl-Alt-A", mac: "Ctrl-Alt-A" }
    // findAll                        --  Find all   --  Object { win: "Ctrl-Alt-K", mac: "Ctrl-Alt-G" }
    // runQuery                       --  undefined   --  Object { win: "CommandOrCtrl-Enter", mac: "Command-Enter" }
    // selectMoreAfter                --  Select more after   --  Object { win: "CommandOrCtrl-D", mac: "Command-D" }
    // selectMoreBefore               --  Select more before   --  Object { win: "CommandOrCtrl-Shift-D", mac: "Command-Shift-D" }

};

export default { global, editor, register, unregister };