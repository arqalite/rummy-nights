// This is the "Offline copy of pages" service worker

const CACHE = "rummy_nights-0.3";

importScripts('https://storage.googleapis.com/workbox-cdn/releases/5.1.2/workbox-sw.js');

self.addEventListener("message", (event) => {
  if (event.data && event.data.type === "SKIP_WAITING") {
    self.skipWaiting();
  }
});

workbox.routing.registerRoute(
  new RegExp('/*'),
  new workbox.strategies.StaleWhileRevalidate({
    cacheName: CACHE
  })
);

// Add whichever assets you want to precache here:
const PRECACHE_ASSETS = [
  '/img/',
  '/snippets/',
  '/'
]

// Listener for the install event - precaches our assets list on service worker install.
self.addEventListener('install', event => {
  event.waitUntil((async () => {
      const cache = await caches.open(CACHE_NAME);
      cache.addAll(PRECACHE_ASSETS);
  })());
});

self.addEventListener('activate', event => {
event.waitUntil(clients.claim());
});

self.addEventListener('fetch', event => {
event.respondWith(async () => {
    const cache = await caches.open(CACHE_NAME);

    // match the request to our cache
    const cachedResponse = await cache.match(event.request);

    // check if we got a valid response
    if (cachedResponse !== undefined) {
        // Cache hit, return the resource
        return cachedResponse;
    } else {
      // Otherwise, go to the network
        return fetch(event.request)
    };
});
});