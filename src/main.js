let currentFilePath = null;
let markdownContent = '';
let lastRender = '';
let renderTimeout = null;

const markdownInput = document.getElementById('markdown-input');
const previewContent = document.getElementById('preview-content');
const filePathEl = document.getElementById('file-path');
const wordCountEl = document.getElementById('word-count');
const toastEl = document.getElementById('toast');

// Keyboard shortcuts
document.addEventListener('keydown', (e) => {
  if ((e.ctrlKey || e.metaKey) && e.key === 'o') {
    e.preventDefault();
    openFile();
  }
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault();
    saveFile();
  }
  if ((e.ctrlKey || e.metaKey) && e.key === 'p') {
    e.preventDefault();
    exportPdf();
  }
  if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'I') {
    e.preventDefault();
    insertImage();
  }
});

// Resizable divider
const divider = document.getElementById('divider');
const editorPane = document.getElementById('editor-pane');
const previewPane = document.getElementById('preview-pane');
let isDragging = false;

divider.addEventListener('mousedown', (e) => {
  isDragging = true;
  divider.classList.add('active');
  document.body.style.cursor = 'col-resize';
  document.body.style.userSelect = 'none';
});

document.addEventListener('mousemove', (e) => {
  if (!isDragging) return;
  const panes = document.getElementById('panes');
  const rect = panes.getBoundingClientRect();
  let x = e.clientX - rect.left;
  const minW = 200;
  const maxW = rect.width - minW - divider.offsetWidth;
  x = Math.max(minW, Math.min(maxW, x));
  const pct = (x / rect.width) * 100;
  editorPane.style.flex = `0 0 ${pct}%`;
  previewPane.style.flex = `1 1 ${100 - pct}%`;
});

document.addEventListener('mouseup', () => {
  if (isDragging) {
    isDragging = false;
    divider.classList.remove('active');
    document.body.style.cursor = '';
    document.body.style.userSelect = '';
  }
});

// Toast notification
let toastTimeout = null;
function showToast(message, type = 'info') {
  toastEl.textContent = message;
  toastEl.className = `toast ${type}`;
  toastEl.classList.add('show');
  clearTimeout(toastTimeout);
  toastTimeout = setTimeout(() => toastEl.classList.remove('show'), 2500);
}

// Live preview via marked.js
function renderPreview() {
  const text = markdownInput.value;
  if (text === lastRender) return;
  lastRender = text;
  markdownContent = text;

  try {
    const html = marked.parse(text, { breaks: true, gfm: true });
    previewContent.innerHTML = html;
  } catch (err) {
    previewContent.innerHTML = `<p style="color:#ef4444;">Render error: ${err.message}</p>`;
  }

  // Word count
  const words = text.trim() ? text.trim().split(/\s+/).length : 0;
  wordCountEl.textContent = `${words} words`;
}

markdownInput.addEventListener('input', () => {
  clearTimeout(renderTimeout);
  renderTimeout = setTimeout(renderPreview, 16); // ~60fps throttle
});

// ---- Error Dialog ----

async function showErrorDialog(title, message) {
  try {
    await invoke('show_error', { title, message });
  } catch (_) {
    // Fallback if dialog fails
    alert(`${title}\n\n${message}`);
  }
}

// ---- Tauri Operations ----

async function openFile() {
  try {
    const result = await invoke('pick_markdown_file');
    if (!result) return;
    currentFilePath = result.path;
    markdownInput.value = result.content;
    lastRender = '';
    renderPreview();
    filePathEl.textContent = currentFilePath;
    filePathEl.title = currentFilePath;
    showToast('File opened', 'success');
  } catch (err) {
    await showErrorDialog('Open Failed', String(err));
  }
}

async function saveFile() {
  let path = currentFilePath;
  if (!path) {
    try {
      path = await invoke('pick_markdown_save_path');
    } catch (err) {
      await showErrorDialog('Save Failed', String(err));
      return;
    }
  }
  if (!path) return;
  try {
    await invoke('write_file', { path, content: markdownContent });
    currentFilePath = path;
    filePathEl.textContent = currentFilePath;
    filePathEl.title = currentFilePath;
    showToast('File saved', 'success');
  } catch (err) {
    await showErrorDialog('Save Failed', String(err));
  }
}

// ---- Image Insertion ----

async function insertImage() {
  try {
    const result = await invoke('pick_image');
    if (!result) return;
    const md = `![image](data:${result.mime};base64,${result.data})`;
    const ta = markdownInput;
    const start = ta.selectionStart;
    const end = ta.selectionEnd;
    const before = ta.value.substring(0, start);
    const after = ta.value.substring(end);
    ta.value = before + md + after;
    ta.selectionStart = ta.selectionEnd = start + md.length;
    ta.dispatchEvent(new Event('input', { bubbles: true }));
    showToast('Image inserted', 'success');
  } catch (err) {
    await showErrorDialog('Insert Image Failed', String(err));
  }
}

// ---- Dark Mode ----

function toggleDarkMode() {
  const html = document.documentElement;
  const cur = html.getAttribute('data-theme') || 'light';
  const next = cur === 'dark' ? 'light' : 'dark';
  html.setAttribute('data-theme', next);
  localStorage.setItem('markchini-theme', next);
}

function loadTheme() {
  const saved = localStorage.getItem('markchini-theme') || 'dark';
  document.documentElement.setAttribute('data-theme', saved);
}

// ---- Settings ----

const settings = {
  font: 'serif',
  fontSize: 'medium',
  margin: 'medium',
};

const FONTS = {
  serif: 'Georgia, Cambria, "Times New Roman", Times, serif',
  'sans-serif': '"Helvetica Neue", Arial, Helvetica, sans-serif',
  monospace: '"Courier New", Courier, monospace',
};

const SIZES = {
  small: { base: '9pt', h1: '18pt', h2: '14pt', h3: '11pt', h4: '10pt', code: '7.5pt', px: '13px' },
  medium: { base: '11pt', h1: '24pt', h2: '18pt', h3: '14pt', h4: '12pt', code: '9.5pt', px: '16px' },
  large: { base: '13pt', h1: '28pt', h2: '22pt', h3: '18pt', h4: '15pt', code: '11pt', px: '19px' },
};

const MARGINS = {
  small: { t: '10mm', r: '15mm', b: '10mm', l: '15mm' },
  medium: { t: '20mm', r: '25mm', b: '20mm', l: '25mm' },
  large: { t: '30mm', r: '35mm', b: '30mm', l: '35mm' },
};

function applySettingsToPreview() {
  previewContent.style.fontFamily = FONTS[settings.font];
  previewContent.style.fontSize = SIZES[settings.fontSize].px;
}

// ---- PDF Export ----

async function exportPdf() {
  if (!markdownContent.trim()) {
    showToast('Nothing to export', 'error');
    return;
  }

  showToast('Generating PDF...', 'info');

  const m = MARGINS[settings.margin];

  try {
    await invoke('compile_pdf', {
      markdown: markdownContent,
      font: settings.font,
      fontSize: settings.fontSize,
      marginT: m.t,
      marginR: m.r,
      marginB: m.b,
      marginL: m.l,
    });
    showToast('PDF exported successfully', 'success');
  } catch (err) {
    await showErrorDialog('Export Failed', String(err));
  }
}

// ---- Initialisation ----

loadTheme();
renderPreview();
applySettingsToPreview();

document.getElementById('setting-font').addEventListener('change', (e) => {
  settings.font = e.target.value;
  applySettingsToPreview();
});
document.getElementById('setting-size').addEventListener('change', (e) => {
  settings.fontSize = e.target.value;
  applySettingsToPreview();
});
document.getElementById('setting-margin').addEventListener('change', (e) => {
  settings.margin = e.target.value;
});
document.getElementById('btn-open').addEventListener('click', openFile);
document.getElementById('btn-save').addEventListener('click', saveFile);
document.getElementById('btn-export').addEventListener('click', exportPdf);
document.getElementById('btn-dark-mode').addEventListener('click', toggleDarkMode);
document.getElementById('btn-image').addEventListener('click', insertImage);
