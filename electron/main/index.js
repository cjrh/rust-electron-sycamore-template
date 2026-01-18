const { app, BrowserWindow, ipcMain } = require('electron');
const path = require('node:path');

// Suppress GPU VSync errors on Linux
app.commandLine.appendSwitch('disable-gpu-vsync');

// Disable Autofill features to prevent DevTools protocol errors
app.commandLine.appendSwitch('disable-features', 'AutofillServerCommunication');

// Load the Neon backend
let rustBackend;
try {
  rustBackend = require('../../crates/backend');
  console.log('Neon backend loaded successfully');
} catch (err) {
  console.warn('Neon backend not available:', err.message);
  console.warn('Run "npm run build:backend" to build the Neon addon');
  rustBackend = null;
}

// IPC handlers for Rust backend
ipcMain.handle('rust-hello', async () => {
  if (rustBackend) {
    return rustBackend.hello();
  }
  return 'Neon backend not loaded - build with "npm run build:backend"';
});

ipcMain.handle('rust-add', async (event, a, b) => {
  if (rustBackend) {
    return rustBackend.add(a, b);
  }
  return null;
});

const createWindow = () => {
  const win = new BrowserWindow({
    width: 900,
    height: 700,
    webPreferences: {
      preload: path.join(__dirname, '../preload/index.js'),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: false, // Required for preload script
    },
    backgroundColor: '#1a1a2e',
  });

  // Load the Trunk-built WASM app
  win.loadFile(path.join(__dirname, '../renderer/index.html'));

  // Open DevTools in development
  if (process.env.NODE_ENV === 'development') {
    win.webContents.openDevTools();
  }
};

app.whenReady().then(() => {
  createWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
