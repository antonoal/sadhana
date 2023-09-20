/* Source: https://github.com/sehgalsakshi/Offline-POST-PWA---Service-Worker/tree/master */
var CACHE = {
    name: 'sadhana-pro',
    version: '-v1'
};

const diaryDayPutUrlR = new RegExp('.*/api/diary/\\d{4}-\\d{2}-\\d{2}/entry');
const dateR = /(\d{4}-\d{2}-\d{2})/;
const cacheTtlDays = 10;

self.addEventListener('install', function (e) {
    console.info('Event: Install');

    // Skip over the "waiting" lifecycle state, to ensure that our
    // new service worker is activated immediately, even if there's
    // another tab open controlled by our older service worker code.
    self.skipWaiting();
});

self.addEventListener('install', function (e) {
    console.info('Event: Activating');

    clearStaleCache();
});

self.addEventListener('fetch', (event) => {
    if (event.request.method === 'GET') {
        // Open the cache
        event.respondWith(sendOfflinePostRequestsToServer()
            .catch((e) => console.error(e))
            .then(() => caches.open(CACHE.name + CACHE.version))
            .then(async (cache) => {
                // Go to the network first
                try {
                    const fetchedResponse = await fetch(event.request.clone(), { credentials: 'same-origin' });
                    cache.put(event.request, fetchedResponse.clone());
                    return fetchedResponse;
                } catch {
                    return await cache.match(event.request, { ignoreVary: true });
                }
            }));
    } else if (event.request.method === 'PUT' && diaryDayPutUrlR.test(event.request.url)) {
        event.respondWith(Promise.resolve().then(async () => {
            // Go to the network first
            try {
                const x = await sendOfflinePostRequestsToServer().catch((e) => console.error(e));
                const fetchedResponse = await fetch(event.request.clone(), { credentials: 'same-origin' });
                // If successful post cached puts
                return fetchedResponse;
            } catch {
                console.info('Saving %s into DB for later processing', event.request.url);
                var authHeader = event.request.headers.get('Authorization');
                var reqUrl = event.request.url;
                var method = event.request.method;
                Promise.resolve(event.request.text()).then((payload) => {
                    //Update local cache
                    updateDiaryDayCachedGet(reqUrl, authHeader, payload);
                    //save offline requests to indexed db
                    saveIntoIndexedDb(reqUrl, authHeader, method, payload)
                });
                return new Response('null', { headers: { 'Content-Type': 'application/json' }, });
            }
        }));
    } else {
        return;
    }
});

async function clearStaleCache() {
    return caches.open(CACHE.name + CACHE.version).then(cache => {
        var staleCobThreshold = new Date();
        staleCobThreshold.setDate(staleCobThreshold.getDate() - cacheTtlDays);

        cache.keys().then(reqs => reqs.forEach(req => {
            const dateStr = req.url.match(dateR);
            if (dateStr && Date.parse(dateStr[1]) < staleCobThreshold) {
                cache.delete(req);
            }
        }))
    });
}

const fetchResponseFromCache = (request) =>
    caches.open(CACHE.name + CACHE.version).then(cache =>
        cache.match(request, { ignoreVary: true }).then(response => returnResponseFromCache(request, response, cache))
    );

const cacheRequest = request => caches.open(CACHE.name + CACHE.version).then(cache =>
    fetch(request.clone(), {
        credentials: 'same-origin'
    }).then(response =>
        cacheResponse(cache, request.clone(), response))
);

async function cacheResponse(cache, request, response) {
    var responseToCache;
    // console.info('Caching request %s', request.url);
    try {
        responseToCache = response.clone();
        cache.put(request, responseToCache);
    } catch (err) {
    }
    return response;
}

async function returnResponseFromCache(request, response, cache) {
    // console.info('Caching request %s', request.url);
    if (!!response) {
        return response;
    } else {
        // console.log(request.url + ' not yet cached!')
        return fetch(request, { credentials: 'same-origin' }).then(response => cacheResponse(cache, request, response))
    }
}

async function getResponseData(data) {
    let promise = Promise.resolve(data).then((text) => {
        return text
    })
    let result = await promise;
    return result
}

function saveIntoIndexedDb(url, authHeader, method, payload) {
    var myRequest = {};
    jsonPayLoad = JSON.parse(payload)
    //add payload if required. If not skip parsing json and stringifying it again
    //jsonPayLoad['eventTime'] = getCurrentTimeString(eventTime)
    myRequest.url = url;
    myRequest.method = method;
    myRequest.authHeader = authHeader;
    myRequest.payload = JSON.stringify(jsonPayLoad);
    var request = indexedDB.open("SadhanaProPostDB");
    request.onsuccess = function (event) {
        var db = event.target.result;
        var tx = db.transaction('postrequest', 'readwrite');
        var store = tx.objectStore('postrequest');
        store.add(myRequest)
    }
}

const sequencePromises = async (promises) => {
    for (const p of promises) {
        await p
    }
};

async function sendOfflinePostRequestsToServer() {
    return new Promise(function (yes, no) {
        console.info('Posting offline writes to the server');
        var request = indexedDB.open("SadhanaProPostDB");
        request.onupgradeneeded = function (event) {
            var db = event.target.result;
            db.onerror = function (event) {
                console.log("Why didn't you allow my web app to use IndexedDB?!");
            };

            var objectStore;
            if (!db.objectStoreNames.contains('postrequest')) {
                objectStore = db.createObjectStore("postrequest", { keyPath: 'id', autoIncrement: true });
            }
            else {
                objectStore = db.objectStoreNames.get('postrequest');
            }
        }
        request.onsuccess = function (event) {
            var db = event.target.result;
            var tx = db.transaction('postrequest', 'readwrite');
            var store = tx.objectStore('postrequest');
            var allRecords = store.getAll();
            allRecords.onsuccess = function () {
                if (allRecords.result && allRecords.result.length > 0) {
                    const postPromises = allRecords.result.map((record) =>
                        fetch(record.url, {
                            method: record.method,
                            headers: {
                                'Accept': 'application/json',
                                'Content-Type': 'application/json',
                                'Authorization': record.authHeader
                            },
                            body: record.payload
                        }).catch((e) => {
                            // Fetch fails only in case of network error. Fetch is successful in case of any response code
                            // console.debug('Exception while sending post request to server' + e);
                            saveIntoIndexedDb(record.url, record.authHeader, record.method, record.payload)
                        })
                    );

                    for (var i = 0; i < allRecords.result.length; i++)
                        store.delete(allRecords.result[i].id)

                    sequencePromises(postPromises).then(() => yes());
                } else {
                    yes();
                }
            };
        }
    });
}

async function updateDiaryDayCachedGet(reqUrl, authHeader, payloadText) {
    var getReq = new Request(reqUrl.replace('/entry', ''));
    getReq.mode = 'cors'
    getReq.headers = { 'Authorization': authHeader }
    var resp = await fetchResponseFromCache(getReq)
    if (resp) {
        var respData = await getResponseData(resp.json());
        var payload = JSON.parse(payloadText);
        respData.diary_day.forEach((item, i) => {
            if (item.practice === payload.entry.practice) respData.diary_day[i] = payload.entry;
        });
        caches.open(CACHE.name + CACHE.version).then(cache => cache.put(getReq, new Response(JSON.stringify(respData))));
    } //TODO: if not found, make a new entry based on another date's cache
    //TODO: update /incomplete-days cache
}