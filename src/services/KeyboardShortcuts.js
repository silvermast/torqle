// import { register, unregister } from '@tauri-apps/plugin-global-shortcut';
import { platform } from '@tauri-apps/plugin-os';

let isMac = false;
try {
    isMac = platform() === 'macos';
} catch (e) {
    console.error('Error determining platform:', e);
}
const prefersCmd = isMac;

document.addEventListener('keydown', (e) => {
    console.log('Keydown:', e);
    console.log('shortcuts:', shortcuts);
    for (const i in shortcuts) {
        const keyCombo = shortcuts[i];
        if (keyCombo.matchesEvent(e)) {
            e.stopPropagation();
            e.preventDefault();
            console.log('Shortcut:', i, keyCombo);
            if (!!keyCombo.handler) {
                keyCombo.handler.call();
            }
            return false;
        }
    }
    return false;
});

/**
 * The long-term plan for this class is to make it configurable by the user.
 * 
 * Be aware of 3 types of shortcuts, based on context:
 * - Global (tauri register/unregister are all global, cross-window)
 * - Window
 * - Editor
 * 
 * The default Tauri menu bar includes a CmdOrControl+W mapped to "Close Window"
 * However, we want to override this to "Close Tab". Tauri makes it really difficult to remove the event handler for this.
 * Fortunately, we can prevent the document event listener from sending the event and intercept them from that point.
 * 
 * Warning: This class currently only supports single-key shortcuts
 */
class KeyCombo {
    // initial constructor vars
    keyList = [];

    keyRegexp = null;
    withCtrl = false;
    withCmd = false;
    withAlt = false;
    withShift = false;

    handler = null;

    constructor(keyList) {
        this.keyList = keyList;

        for (const key of keyList) {
            switch (key) {
                case 'CmdOrControl':
                case 'CommandOrControl':
                    if (prefersCmd) {
                        this.withCmd = true;
                    } else {
                        this.withCtrl = true;
                    }
                    break;

                case 'Control':
                    this.withCtrl = true;
                    break;

                case 'Command':
                case 'Cmd':
                    this.withCmd = true;
                    break;

                case 'Shift':
                    this.withShift = true;
                    break;

                case 'Alt':
                    this.withAlt = true;
                    break;

                default:
                    this.keyRegexp = new RegExp(`^${key}$`, 'i');
            }
        }
    }

    register(handler) {
        this.handler = handler;
    }

    unregister() {
        this.handler = null;
    }

    matchesEvent(e) {
        return this.withCtrl === e.ctrlKey
            && this.withCmd === e.metaKey
            && this.withAlt === e.altKey
            && this.withShift === e.shiftKey
            && this.keyRegexp.test(e.key);
    }

    /**
     * Formats the key binding into a tauri-compatible string
     * @returns String
     */
    formatForTauri() {
        return this.keyList.join('+');
    }

    /**
     * Formats shortcut for Ace
     * @return Object ({ mac, win })
     * @example { mac: 'Command-Shift-D', win: 'Ctrl-Shift-D' }
     */
    formatForAce() {
        let keys = this.keyList.join('-').replace(/Control/, 'Ctrl');
        return {
            win: keys.replace(/CommandOrCtrl/, 'Ctrl'),
            mac: keys.replace(/CommandOrCtrl/, 'Command'),
        }
    }
}

const shortcuts = {
    // newWindow: new KeyCombo(['CommandOrControl', 'N']), // defined in tauri default menu

    newTab: new KeyCombo(['CommandOrControl', 'T']),
    prevTab: new KeyCombo(['CommandOrControl', 'Alt', 'ArrowLeft']),
    nextTab: new KeyCombo(['CommandOrControl', 'Alt', 'ArrowRight']),
    
    closeWindow: new KeyCombo(['CommandOrControl', 'Shift', 'W']),
    closeTab: new KeyCombo(['CommandOrControl', 'W']),
    
    filterTables: new KeyCombo(['CommandOrControl', 'J']),
    selectDatabase: new KeyCombo(['CommandOrControl', 'K']),

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

export { shortcuts };