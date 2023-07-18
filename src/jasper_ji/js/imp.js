const { createEditor, createToolbar } = window.wangEditor

const editorConfig = {
    placeholder: 'Type here...',
    onChange(editor) {
        const html = editor.getHtml()
        console.log('editor content', html)
        // 也可以同步到 <textarea>
    }
}

// const editor = createEditor({
//     selector: '#editor-container',
//     html: '<p><br></p>',
//     config: editorConfig,
//     mode: 'default', // or 'simple'
// })

const toolbarConfig = {}
let editor;

// const toolbar = createToolbar({
//     editor,
//     selector: '#toolbar-container',
//     config: toolbarConfig,
//     mode: 'default', // or 'simple'
// })

export function hello() {
    editor = createEditor({
        selector: '#editor-container',
        html: '<p><br></p>',
        config: editorConfig,
        mode: 'default', // or 'simple'
    })
    createToolbar({
        editor,
        selector: '#toolbar-container',
        config: toolbarConfig,
        mode: 'default', // or 'simple'
    })
    return "very important message"
}

export function get_text() {
    return editor.getText();
}

