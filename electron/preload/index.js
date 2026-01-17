const { contextBridge, ipcRenderer } = require('electron');

// Expose Rust backend functions to the renderer process
contextBridge.exposeInMainWorld('electronAPI', {
  // Call the Rust hello function via IPC
  rustHello: () => ipcRenderer.invoke('rust-hello'),

  // Call the Rust add function via IPC
  rustAdd: (a, b) => ipcRenderer.invoke('rust-add', a, b),
});

console.log('Preload script loaded - electronAPI exposed');
