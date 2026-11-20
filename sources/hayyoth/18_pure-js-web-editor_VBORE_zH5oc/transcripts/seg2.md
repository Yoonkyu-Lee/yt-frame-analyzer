# seg2 (2:06 - 5:33) - VBORE_zH5oc - Hayyoth "No AI - Just pure JS hand coding" (web-editor)

Setup notes: macOS-style editor window titled `index.html — web-editor` (no line numbers in the gutter; no save messages visible in this segment), Firefox window at `http://127.0.0.1:5500/index.html` with devtools (Inspector tab, Rules panel). Editor and browser are alternated via macOS window switching (thumbnail on the left edge). Webcam overlay ignored.

## Timeline
- [02:06] Editor, `index.html — web-editor`. File is a copied W3Schools-style "Two Equal Columns" float demo (no `<!DOCTYPE>` visible in the scrolled view; top is `<html>`). Mouse at left edge, about to switch to Firefox.
- [02:07] Firefox: page "Two Equal Columns" h2, two grey float columns ("Column 1" / "Some text..", "Column 2" / "Some text.."), 300px tall, default 8px body margin.
- [02:08] DevTools opened (Inspector). DOM: `<!DOCTYPE html>`, `<html event scroll>`, `<head>`, `<body>` selected with `<h2>Two Equal Columns</h2>` and `<div class="row">`. Rules: `* { box-sizing: border-box; }` from `inline:2`.
- [02:11] Devtools pane enlarged; viewport readout `801.0px x 421.0px`.
- [02:17] Browser window widened; hovering `<body>` highlights body box `872 x 348.717` with the default 8px margin (yellow).
- [02:19] In Rules, adds property to `element {}` (body): typing `margin` (autocomplete list margin, margin-block, margin-block-end ...).
- [02:25] Body inline style now `margin: 0; height: 100vh;` (node `<body style="margin: 0;height: 100vh;">`, body highlight `888 x 421`). Content now flush to the left edge.
- [02:26-02:27] Cursor over rules; both declarations (`margin: 0;` `height: 100vh;`) selected to copy.
- [02:28-02:29] Switches to editor (still original file, top at `<html>`).
- [02:50] Editor mid window-animation; file now starts with `<!DOCTYPE html>` / `<html>` / blank / `<head>` and the CSS is re-indented one extra level inside `<head>`/`<style>`. Pasted lines partly visible (`n: 0;`, `t: 100vh;`). `height: 300px;` comment `/* Should be removed. Only for demonstration */` now wrapped on its own line.
- [02:51] Firefox: Rules show `body { margin: 0; height: 100vh; }` (source `inline`) - pasted rule applied after reload.
- [03:05] Editor: `body { margin: 0; height: 100vh; }` rule present after the `* { box-sizing }` rule. Cursor at closing `}` of `.column`.
- [03:06] Scrolled down: `<body>` now contains `<div class="nav"></div>` followed by `<div class="row">` with two `.column` divs (inline `style="background-color: #aaa;"` / `#bbb;`). Old `<h2>Two Equal Columns</h2>` is gone. Word `row` highlighted.
- [03:07-03:08] Firefox: DOM `<div class="nav"></div>` (empty) + `<div class="row">`; columns start at very top of page.
- [03:20] Editor: nav now `<div class="nav">` / `<div class="title">Web Editor</div>` / `</div>` (word `title` selected).
- [03:21] Firefox: "Web Editor" text appears top-left in default serif; nav now has children.
- [03:23] Inspector hover on `div.nav`: `888 x 19.2`.
- [03:29-03:32] Live-edits nav height in the inline style via Rules: `height: 62px` -> `5` -> `52px` (testing values).
- [03:37] nav style `height: 42px;display: ;` - adding `display:` (autocomplete: block, block ruby, contents, flex, flow-root, flow-root list-item, grid).
- [03:47] nav style `height: 42px;display: flex;align-items: center;padding: 12px;background: aqua;` - cyan bar visible, "Web Editor" padded and vertically centered.
- [03:49-03:55] Cycles background colors using autocomplete/arrow keys: aqua -> aliceblue -> violet -> wheat. Settles on `wheat`.
- [03:57] Final nav inline style in Rules: `height: 42px; display: flex; align-items: center; padding: 12px; background: wheat;` (all five selected to copy).
- [03:58] Editor (cursor still in `title` line); about to paste.
- [04:04] Editor: new rule after `body {...}`:
  `.nav { height: 42px; display: flex; align-items: center; padding: 12px; background: wheat; }` (one declaration per line; color swatch shown beside `wheat`).
- [04:05] Firefox: reload; DOM `<div class="nav" flex>` expanded with `<div class="title">Web Editor</div>`; Rules `.nav {...}`.
- [04:11-04:13] Selects `div.row` and adds inline `height: 100%;`: `<div class="row" style="height: 100%;">` - body gets overflow badge; hover shows body `888 x 421`.
- [04:15] Body rules: `margin: 0; height: 100vh; display: flex;` -> layout breaks: nav squeezed to content width on left, columns narrow next to it.
- [04:19] Adds `flex-flow: column;` (autocomplete column, column-reverse). Layout restores: nav on top, columns below.
- [04:21] `div.row` hover: `888 x 379` `Flex Item` (fills remaining 421-42).
- [04:23] Body rules: `margin: 0; height: 100vh; display: flex; flex-flow: column;`.
- [04:28] Selects `display: flex; flex-flow: column;` to copy.
- [04:29-04:30] Editor: cursor at end of `height: 100vh;` in body rule.
- [04:32] Editor: body rule now has `display: flex;` and `flex-flow: column;` added after `height: 100vh;`.
- [04:33] Firefox DOM: `<body flex>`, `<div class="nav">...</div flex>`, `<div class="row">`; Rules body shows 4 props.
- [04:42] `div.row` with inline `height: 100%;` (hover `888 x 379 Flex Item`).
- [04:45-04:47] `height: 100%;` selected, then copied.
- [04:59] Editor: body + `.nav` rules visible, scroll at top.
- [05:00] Editor scrolled: new rule after `.nav`: `.row { height: 100%; }`; and in `.column`, `height: 300px;` changed to `height: 100%;` (autocomplete tooltip `100%` showing). Demo comment still on next line.
- [05:01] macOS window-switch animation editor -> Firefox.
- [05:02-05:03] Firefox: columns now fill height to bottom of viewport; DOM `div.row` expanded with `div.column style="background-color:#aaa;"` and `#bbb`; Rules `.row { height: 100%; }`.
- [05:03] Editor: `.column` `height: 100%;` confirmed; cursor on `padding: 10px;` line.
- [05:26] Editor scrolled to bottom: first column's `<h2>Column 1</h2>` + `<p>Some text..</p>` replaced by `<textarea></textarea>` (cursor between `<textarea` and `>`).
- [05:27] Firefox: left column shows a small default textarea with resize grip at top-left; column 2 unchanged.
- [05:28] Mouse dragging the textarea resize grip.
- [05:33] Inspector: `<textarea style="width: 269px; height: 130px;"></textarea>` after resize; hover readout `textarea 269 x 130`. Rules `element { width: 269px; height: 130px; }`.

## File states
No editor line numbers are visible in the gutter; the `n|` numbers below are my reconstruction of the file at the END of the segment (~05:33), not editor numbers. Combined from views at 03:05, 04:59, 05:00, 05:26. Exact blank-line placement inside `<body>` seen at 05:26 (blank line after `<body>` with indent guide, blank line before `</body>`, blank between `</body>` and `</html>`).

```
 1| <!DOCTYPE html>
 2| <html>
 3| 
 4| <head>
 5|   <meta name="viewport" content="width=device-width, initial-scale=1">
 6|   <style>
 7|     * {
 8|       box-sizing: border-box;
 9|     }
10| 
11|     body {
12|       margin: 0;
13|       height: 100vh;
14|       display: flex;
15|       flex-flow: column;
16|     }
17| 
18|     .nav {
19|       height: 42px;
20|       display: flex;
21|       align-items: center;
22|       padding: 12px;
23|       background: wheat;
24|     }
25| 
26|     .row {
27|       height: 100%;
28|     }
29| 
30|     /* Create two equal columns that floats next to each other */
31|     .column {
32|       float: left;
33|       width: 50%;
34|       padding: 10px;
35|       height: 100%;
36|       /* Should be removed. Only for demonstration */
37|     }
38| 
39|     /* Clear floats after the columns */
40|     .row:after {
41|       content: "";
42|       display: table;
43|       clear: both;
44|     }
45|   </style>
46| </head>
47| 
48| <body>
49| 
50|   <div class="nav">
51|     <div class="title">Web Editor</div>
52|   </div>
53| 
54|   <div class="row">
55|     <div class="column" style="background-color: #aaa;">
56|       <textarea></textarea>
57|     </div>
58|     <div class="column" style="background-color: #bbb;">
59|       <h2>Column 2</h2>
60|       <p>Some text..</p>
61|     </div>
62|   </div>
63| 
64| </body>
65| 
66| </html>
```

Original state at 02:06 (before edits), as seen: `<html>`, `<head>`, `<meta name="viewport" content="width=device-width, initial-scale=1">`, `<style>`, `* {` `box-sizing: border-box;` `}`, blank, `/* Create two equal columns that floats next to each other */`, `.column { float: left; width: 50%; padding: 10px; height: 300px; /* Should be removed. Only for demonstration */ }` (height line and comment on ONE line), blank, `/* Clear floats after the columns */`, `.row:after { content: ""; display: table; clear: both; }`, `</style>`, `</head>`, `<body>`, blank, `<h2>Two Equal Columns</h2>`, blank, `<div class="row">` ... (unindented 1-level).

Intermediate body (03:06, before nav styling):
```
<body>

  <div class="nav"></div>

  <div class="row">
    <div class="column" style="background-color: #aaa;">
      <h2>Column 1</h2>
      <p>Some text..</p>
    </div>
    <div class="column" style="background-color: #bbb;">
      <h2>Column 2</h2>
      ?? (rest not visible at that frame)
```

## Shell & outputs
No terminal in this segment. Browser (Firefox, `http://127.0.0.1:5500/index.html`) renders:
- 02:07: heading "Two Equal Columns"; two grey columns (#aaa left, #bbb right) with "Column 1"/"Column 2" and "Some text..".
- 03:21: "Web Editor" plain serif at top-left, then two columns.
- 03:47-03:55: nav bar coloured aqua, aliceblue, violet, then wheat.
- 04:15: body `display:flex` without column flow: "Web Editor" nav squeezed on left, narrow columns beside it.
- 05:02: wheat nav bar "Web Editor", columns extend to bottom of viewport.
- 05:27-05:33: left column contains a textarea (default small, then 269 x 130 px); right column "Column 2 / Some text..".
DevTools Inspector/Rules text is quoted in the timeline.

## Observations
- Starts from a copied W3Schools "Two Equal Columns" float layout (comments kept, including "Should be removed. Only for demonstration") and reshapes it into an editor skeleton: top nav bar (`.nav` with `.title` "Web Editor") plus a full-height two-column row (left gets a textarea, right presumably the file manager).
- Workflow: prototype CSS live in Firefox devtools Rules panel (trying heights 62/52/42px, cycling colors with arrow keys), then select/copy declarations into a new rule in the editor, then reload. Devtools as scratchpad, editor as source of truth.
- Layout decision: body `display:flex; flex-flow:column; height:100vh; margin:0`, `.nav` fixed 42px, `.row` `height:100%`; columns' demo `height:300px` swapped for `100%` (the stale demo comment is left behind).
- First content change in the DOM: replace Column 1 contents with a `<textarea>` (the editor surface), resized by dragging (269 x 130 px) in the browser.
- No JavaScript written yet in this segment; no save messages seen.
