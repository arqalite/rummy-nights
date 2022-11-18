// This is the "Offline copy of pages" service worker

const CACHE = "rummy_nights-0.3";

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