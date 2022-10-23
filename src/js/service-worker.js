const cacheName = "rummy-nights-v0.1.1"
const appShellFiles = [
    '/img/add-player.svg',
    '/img/arrow.svg',
    '/img/back.svg',
    '/img/exit.svg',
    '/img/favicon.ico',
    '/img/fly-away.svg',
    '/img/home.svg',
    '/img/intro.gif',
    '/img/intro_bak.gif',
    '/img/logo_192.png',
    '/img/logo_512.png',
    '/img/menu.svg',
    '/img/menu_button.svg',
    '/img/new.svg',
    '/img/remove.svg',
    '/img/resume.svg',
    '/img/save.svg',
    '/img/share.svg',
    '/img/trophy.svg',
    '/img/user.svg',
    '/app.webmanifest',
    '/favicon.ico',
    '/index.html',
    '/rummy-nights.js',
    '/rummy-nights_bg.wasm',
    '/service-worker.js',
    '/style.css'
];

self.addEventListener('install', (e) => {
    console.log('[Service Worker] Install');
    e.waitUntil((async () => {
      const cache = await caches.open(cacheName);
      console.log('[Service Worker] Caching all: app shell and content');
      await cache.addAll(appShellFiles);
    })());
  });

  self.addEventListener('fetch', (e) => {
    e.respondWith((async () => {
      const r = await caches.match(e.request);
      console.log(`[Service Worker] Fetching resource: ${e.request.url}`);
      if (r) { return r; }
      const response = await fetch(e.request);
      const cache = await caches.open(cacheName);
      console.log(`[Service Worker] Caching new resource: ${e.request.url}`);
      cache.put(e.request, response.clone());
      return response;
    })());
  });

  self.addEventListener('activate', (e) => {
    e.waitUntil(caches.keys().then((keyList) => {
      return Promise.all(keyList.map((key) => {
        if (key === cacheName) { return; }
        return caches.delete(key);
      }));
    }));
  });
