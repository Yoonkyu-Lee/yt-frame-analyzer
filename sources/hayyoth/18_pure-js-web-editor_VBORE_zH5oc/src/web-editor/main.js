const btnRun = document.getElementById("btnRun");

const render = document.getElementById("render");
//const iframedoc = render.contentDocument || render.contentWindow.document;

const editor = ace.edit("editor");
editor.setTheme("ace/theme/monokai");
editor.session.setMode("ace/mode/html");

btnRun.addEventListener("click", () => {
    render.srcdoc = editor.getValue();
})
