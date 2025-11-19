// src/App.tsx
import React, { useEffect, useMemo, useState } from "react";
import Dashboard from "./components/Dashboard/Dashboard";
import Settings from "./components/Settings/Settings";
import { ActivityLog, AiSettings } from "./types";
import { invokeCommand } from "./utils/tauri";

function App(): JSX.Element {
  const [logs, setLogs] = useState<ActivityLog[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string>();
  const [latestReport, setLatestReport] = useState<string>();
  const [reportError, setReportError] = useState<string>();
  const [isGeneratingReport, setIsGeneratingReport] = useState<boolean>(false);
  const [activeView, setActiveView] = useState<"dashboard" | "settings">("dashboard");
  const [aiSettings, setAiSettings] = useState<AiSettings | null>(null);

  useEffect(() => {
    const fetchLogs = async () => {
      setIsLoading(true);
      setError(undefined);
      const today = new Date().toISOString().slice(0, 10);

      try {
        const response = await invokeCommand<ActivityLog[]>("get_logs_by_date", { date: today });
        setLogs(response);
      } catch (err) {
        console.error("Failed to fetch logs", err);
        setError(err instanceof Error ? err.message : String(err));
      } finally {
        setIsLoading(false);
      }
    };

    fetchLogs();
  }, []);

  useEffect(() => {
    const loadSettings = async () => {
      try {
        const response = await invokeCommand<{ 
          provider_url: string; 
          api_key?: string | null;
          model_name: string;  // ✨ ADD THIS
        }>("fetch_ai_settings");
        setAiSettings({
          providerUrl: response.provider_url,
          apiKey: response.api_key ?? "",
          model_name: response.model_name,  // ✨ ADD THIS
        });
      } catch (err) {
        console.error("Failed to load AI settings", err);
      }
    };

    loadSettings();
  }, []);

  const handleGenerateReport = async () => {
    setReportError(undefined);
    setIsGeneratingReport(true);
    try {
      const summary = await invokeCommand<string>("trigger_manual_summary");
      setLatestReport(summary);
    } catch (err) {
      console.error("Failed to generate report", err);
      setReportError(err instanceof Error ? err.message : String(err));
    } finally {
      setIsGeneratingReport(false);
    }
  };

  const recentEvents = useMemo(() => logs.slice(0, 5), [logs]);

  return (
    <div className="min-h-screen bg-slate-950 p-8 text-slate-100">
      <header className="mb-6 flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
        <div>
          <h1 className="text-3xl font-bold text-slate-50">DevChronicle</h1>
          <p className="text-sm text-slate-400">
            Hybrid desktop hub for ingesting developer activity and generating AI summaries.
          </p>
        </div>
        <nav className="flex items-center gap-2">
          <button
            type="button"
            onClick={() => setActiveView("dashboard")}
            className={`rounded-md px-4 py-2 text-sm font-medium transition ${
              activeView === "dashboard"
                ? "bg-blue-500/20 text-blue-100"
                : "bg-slate-900/60 text-slate-300 hover:bg-slate-900"
            }`}
          >
            Dashboard
          </button>
          <button
            type="button"
            onClick={() => setActiveView("settings")}
            className={`rounded-md px-4 py-2 text-sm font-medium transition ${
              activeView === "settings"
                ? "bg-blue-500/20 text-blue-100"
                : "bg-slate-900/60 text-slate-300 hover:bg-slate-900"
            }`}
          >
            Settings
          </button>
        </nav>
      </header>

      {activeView === "dashboard" ? (
        <>
          <Dashboard
            logs={logs}
            isLoading={isLoading}
            error={error}
            onGenerateReport={handleGenerateReport}
            isGeneratingReport={isGeneratingReport}
            latestReport={latestReport}
            reportError={reportError}
          />

          <section className="mt-8 rounded-lg border border-slate-800 bg-slate-900/60 p-4">
            <header className="flex items-center justify-between">
              <h2 className="text-xl font-semibold">Debug: Recent Events</h2>
              <span className="text-sm text-slate-400">Total logs today: {logs.length}</span>
            </header>
            {aiSettings && (
              <p className="mt-2 text-xs text-slate-500">
                Using AI provider at <span className="font-semibold text-slate-300">{aiSettings.providerUrl}</span>
                {/* ✨ ADD MODEL INFO */}
                {" "}with model <span className="font-semibold text-slate-300">{aiSettings.model_name}</span>
              </p>
            )}
            {recentEvents.length === 0 && !isLoading && (
              <p className="mt-2 text-sm text-slate-400">No logs returned for today.</p>
            )}
            <ul className="mt-4 space-y-2">
              {recentEvents.map((log) => (
                <li key={log.id} className="rounded-md border border-slate-800 bg-slate-950/70 p-3">
                  <div className="flex items-center justify-between">
                    <span className="font-medium text-slate-200">{log.source}</span>
                    <time className="text-xs text-slate-400">
                      {new Date(log.timestamp).toLocaleTimeString()}
                    </time>
                  </div>
                  <pre className="mt-2 overflow-x-auto text-xs text-slate-300">
                    {JSON.stringify(log.payload, null, 2)}
                  </pre>
                </li>
              ))}
            </ul>
          </section>
        </>
      ) : (
        <Settings
          onSettingsSaved={(updated) => {
            setAiSettings(updated);
            setActiveView("dashboard");
          }}
        />
      )}
    </div>
  );
}

export default App;
