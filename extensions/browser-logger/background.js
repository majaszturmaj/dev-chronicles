const DEFAULT_ENDPOINT = "http://localhost:3030";
const dwellTimers = new Map();
const pageStartTimes = new Map();

async function sendEvent(payload) {
  const endpoint = DEFAULT_ENDPOINT.replace(/\/$/, "") + "/ingest/browser";

  try {
    const response = await fetch(endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        source: "browser",
        payload: payload
      })
    });
    
    if (!response.ok) {
      console.error("DevChronicle Browser Logger: server returned error", response.status);
    }
  } catch (error) {
    console.error("DevChronicle Browser Logger: failed to send event", error);
  }
}

async function scheduleCapture(tabId, tab) {
  if (!tab || !tab.active || !tab.url || !tab.url.startsWith("http")) {
    return;
  }

  clearTimeout(dwellTimers.get(tabId));
  
  // Record when user started viewing this page
  const startTime = Date.now();
  pageStartTimes.set(tabId, startTime);

  // Fetch server configuration (dwell interval) and use it when scheduling
  let delayMs = 5000; // fallback
  try {
    const cfgRes = await fetch((DEFAULT_ENDPOINT.replace(/\/$/, "") + "/ingest/config"));
    if (cfgRes.ok) {
      const cfg = await cfgRes.json();
      if (cfg && cfg.extension_send_interval_ms) {
        delayMs = parseInt(cfg.extension_send_interval_ms, 10) || delayMs;
      }
    }
  } catch (e) {
    // ignore and use fallback
  }

  const timer = setTimeout(async () => {
    try {
      const currentTab = await chrome.tabs.get(tabId);
      if (!currentTab.active || !currentTab.url?.startsWith("http")) {
        return;
      }

      const timeOnPage = Math.floor((Date.now() - startTime) / 1000);
      
      await sendEvent({
        url: currentTab.url,
        title: currentTab.title || "",
        time_on_page_sec: timeOnPage,
        referrer: currentTab.openerTabId ? (await chrome.tabs.get(currentTab.openerTabId).catch(() => null))?.url : undefined,
        user_agent: navigator.userAgent
      });
    } catch (error) {
      console.error("DevChronicle Browser Logger: unable to inspect tab", error);
    }
  }, delayMs); // Use configured delay before sending

  dwellTimers.set(tabId, timer);
}

chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  if (changeInfo.status === "complete" && tab.active && tab.url?.startsWith("http")) {
    scheduleCapture(tabId, tab);
  }
});

chrome.tabs.onActivated.addListener(({ tabId }) => {
  chrome.tabs.get(tabId, (tab) => scheduleCapture(tabId, tab));
});

chrome.tabs.onRemoved.addListener((tabId) => {
  clearTimeout(dwellTimers.get(tabId));
  dwellTimers.delete(tabId);
  pageStartTimes.delete(tabId);
});

