const DEFAULT_ENDPOINT = "http://localhost:3030";
const dwellTimers = new Map();

async function sendEvent(details) {
  const endpoint = DEFAULT_ENDPOINT.replace(/\/$/, "") + "/ingest";

  try {
    await fetch(endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        source: "browser",
        payload: details
      })
    });
  } catch (error) {
    console.error("DevChronicle Browser Logger: failed to send event", error);
  }
}

function scheduleCapture(tabId, tab) {
  if (!tab || !tab.active || !tab.url || !tab.url.startsWith("http")) {
    return;
  }

  clearTimeout(dwellTimers.get(tabId));

  const timer = setTimeout(async () => {
    try {
      const currentTab = await chrome.tabs.get(tabId);
      if (!currentTab.active || !currentTab.url?.startsWith("http")) {
        return;
      }

      await sendEvent({
        event: "focus",
        url: currentTab.url,
        title: currentTab.title,
        timestamp: new Date().toISOString()
      });
    } catch (error) {
      console.error("DevChronicle Browser Logger: unable to inspect tab", error);
    }
  }, 5000);

  dwellTimers.set(tabId, timer);
}

chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  if (changeInfo.status === "complete") {
    scheduleCapture(tabId, tab);
  }
});

chrome.tabs.onActivated.addListener(({ tabId }) => {
  chrome.tabs.get(tabId, (tab) => scheduleCapture(tabId, tab));
});

chrome.tabs.onRemoved.addListener((tabId) => {
  clearTimeout(dwellTimers.get(tabId));
  dwellTimers.delete(tabId);
});

