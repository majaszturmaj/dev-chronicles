// src/components/Settings/Settings.tsx
import React, { useEffect, useState } from "react";
import { AiSettings } from "../../types";
import { invokeCommand } from "../../utils/tauri";

interface SettingsProps {
  onSettingsSaved?: (settings: AiSettings) => void;
}

interface ExtendedSettings extends AiSettings {
  temperature?: number;
  batch_size?: number;
  summary_frequency_min?: number;
}

const DEFAULT_URL = "http://localhost:1234/v1";
const DEFAULT_MODEL = "gpt-4o-mini";

const Settings: React.FC<SettingsProps> = ({ onSettingsSaved }) => {
  const [isLoading, setIsLoading] = useState(true);
  const [settings, setSettings] = useState<ExtendedSettings>({
    providerUrl: DEFAULT_URL,
    apiKey: "",
    model_name: DEFAULT_MODEL,
    temperature: 0.2,
    batch_size: 100,
    summary_frequency_min: 10,
  });
  const [error, setError] = useState<string>();
  const [successMessage, setSuccessMessage] = useState<string>();
  const [isSaving, setIsSaving] = useState(false);
  const [isTesting, setIsTesting] = useState(false);
  const [testResult, setTestResult] = useState<{ success: boolean; message: string } | null>(null);

  useEffect(() => {
    let isMounted = true;
    const loadSettings = async () => {
      try {
        const response = await invokeCommand<{ 
          provider_url: string; 
          api_key?: string | null;
          model_name: string;
          temperature?: number;
          batch_size?: number;
          summary_frequency_min?: number;
        }>("fetch_ai_settings");
        if (!isMounted) {
          return;
        }
        setSettings({
          providerUrl: response.provider_url || DEFAULT_URL,
          apiKey: response.api_key ?? "",
          model_name: response.model_name || DEFAULT_MODEL,
          temperature: response.temperature ?? 0.2,
          batch_size: response.batch_size ?? 100,
          summary_frequency_min: response.summary_frequency_min ?? 10,
        });
      } catch (err) {
        console.error("Failed to load AI settings", err);
        if (isMounted) {
          setError(err instanceof Error ? err.message : String(err));
        }
      } finally {
        if (isMounted) {
          setIsLoading(false);
        }
      }
    };

    loadSettings();
    return () => {
      isMounted = false;
    };
  }, []);

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value, type } = event.target;
    setSettings((prev) => ({
      ...prev,
      [name]: type === "number" ? parseFloat(value) : value,
    }));
  };

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setError(undefined);
    setSuccessMessage(undefined);
    setIsSaving(true);

    try {
      await invokeCommand("save_ai_settings", {
        settings: {
          provider_url: settings.providerUrl,
          api_key: settings.apiKey || null,
          model_name: settings.model_name,
          temperature: settings.temperature,
          batch_size: settings.batch_size,
          summary_frequency_min: settings.summary_frequency_min,
        }
      });
      setSuccessMessage("Settings saved successfully.");
      setTestResult(null);
      onSettingsSaved?.(settings);
    } catch (err) {
      console.error("Failed to save AI settings", err);
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setIsSaving(false);
    }
  };

  const handleTestConnection = async () => {
    if (settings.providerUrl.trim()) {
      try {
        await invokeCommand("save_ai_settings", {
          settings: {
            provider_url: settings.providerUrl,
            api_key: settings.apiKey || null,
            model_name: settings.model_name,
            temperature: settings.temperature,
            batch_size: settings.batch_size,
            summary_frequency_min: settings.summary_frequency_min,
          }
        });
      } catch (err) {
        console.error("Failed to save settings before test", err);
      }
    }

    setTestResult(null);
    setError(undefined);
    setIsTesting(true);

    try {
      const result = await invokeCommand<string>("test_ai_connection");
      setTestResult({ success: true, message: result });
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setTestResult({ success: false, message: errorMessage });
    } finally {
      setIsTesting(false);
    }
  };

  return (
    <section className="space-y-6 rounded-lg border border-slate-800 bg-slate-900/60 p-6">
      <header>
        <h1 className="text-2xl font-bold text-slate-100">AI Configuration</h1>
        <p className="mt-2 text-sm text-slate-400">
          Configure the DevChronicle AI provider connection. These settings determine where summaries are generated.
        </p>
      </header>

      {isLoading ? (
        <p className="text-sm text-slate-400">Loading settings…</p>
      ) : (
        <form onSubmit={handleSubmit} className="space-y-6">
          <div className="space-y-2">
            <label className="block text-sm font-medium text-slate-200" htmlFor="provider-url">
              AI Provider URL
            </label>
            <input
              id="provider-url"
              name="providerUrl"
              type="url"
              required
              placeholder={DEFAULT_URL}
              value={settings.providerUrl}
              onChange={handleChange}
              className="w-full rounded-md border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500/40"
            />
            <p className="text-xs text-slate-500">
              Examples: https://api.openai.com/v1 or http://localhost:1234/v1 (LM Studio).
            </p>
          </div>

          {/* ✨ NEW FIELD: Model Name */}
          <div className="space-y-2">
            <label className="block text-sm font-medium text-slate-200" htmlFor="model-name">
              Model Name
            </label>
            <input
              id="model-name"
              name="model_name"
              type="text"
              required
              placeholder={DEFAULT_MODEL}
              value={settings.model_name}
              onChange={handleChange}
              className="w-full rounded-md border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500/40"
            />
            <p className="text-xs text-slate-500">
              Examples: gpt-4o-mini, gpt-3.5-turbo, or any model loaded in LM Studio.
            </p>
          </div>

          {/* ✨ NEW FIELD: Temperature (Creativity) */}
          <div className="space-y-2">
            <label className="block text-sm font-medium text-slate-200" htmlFor="temperature">
              Temperature (Creativity): {settings.temperature}
            </label>
            <input
              id="temperature"
              name="temperature"
              type="range"
              min="0"
              max="2"
              step="0.1"
              value={settings.temperature ?? 0.2}
              onChange={handleChange}
              className="w-full"
            />
            <p className="text-xs text-slate-500">
              Lower = more deterministic/focused (0.0-0.3). Higher = more creative/varied (0.7-2.0).
            </p>
          </div>

          {/* ✨ NEW FIELD: Batch Size */}
          <div className="space-y-2">
            <label className="block text-sm font-medium text-slate-200" htmlFor="batch-size">
              Batch Size (logs per summary)
            </label>
            <input
              id="batch-size"
              name="batch_size"
              type="number"
              min="10"
              max="1000"
              value={settings.batch_size ?? 100}
              onChange={handleChange}
              className="w-full rounded-md border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500/40"
            />
            <p className="text-xs text-slate-500">
              How many activity logs to include in each summary (10-1000).
            </p>
          </div>

          {/* ✨ NEW FIELD: Summary Frequency */}
          <div className="space-y-2">
            <label className="block text-sm font-medium text-slate-200" htmlFor="summary-frequency">
              Summary Frequency (minutes)
            </label>
            <input
              id="summary-frequency"
              name="summary_frequency_min"
              type="number"
              min="5"
              max="60"
              value={settings.summary_frequency_min ?? 10}
              onChange={handleChange}
              className="w-full rounded-md border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500/40"
            />
            <p className="text-xs text-slate-500">
              Automatically generate summaries every N minutes (5-60).
            </p>
          </div>

          <div className="space-y-2">
            <label className="block text-sm font-medium text-slate-200" htmlFor="api-key">
              API Key
            </label>
            <input
              id="api-key"
              name="apiKey"
              type="password"
              placeholder="sk-..."
              value={settings.apiKey ?? ""}
              onChange={handleChange}
              className="w-full rounded-md border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500/40"
            />
            <p className="text-xs text-slate-500">
              Leave blank when connecting to a local model that does not require authentication.
            </p>
          </div>

          {error && (
            <p className="text-sm text-red-400" role="alert">
              {error}
            </p>
          )}

          {successMessage && (
            <p className="text-sm text-green-400" role="status">
              {successMessage}
            </p>
          )}

          {testResult && (
            <div
              className={`rounded-md border p-3 ${
                testResult.success
                  ? "border-green-500/50 bg-green-500/10"
                  : "border-red-500/50 bg-red-500/10"
              }`}
            >
              <p
                className={`text-sm ${
                  testResult.success ? "text-green-300" : "text-red-300"
                }`}
                role="alert"
              >
                {testResult.message}
              </p>
            </div>
          )}

          <div className="flex justify-end gap-3">
            <button
              type="button"
              onClick={handleTestConnection}
              disabled={isTesting || isSaving || !settings.providerUrl.trim()}
              className="inline-flex items-center justify-center rounded-md border border-slate-600 bg-slate-800/60 px-4 py-2 text-sm font-medium text-slate-200 transition hover:bg-slate-700/60 disabled:cursor-not-allowed disabled:opacity-60"
            >
              {isTesting ? "Testing…" : "Test Connection"}
            </button>
            <button
              type="submit"
              disabled={isSaving || isTesting}
              className="inline-flex items-center justify-center rounded-md border border-blue-500 bg-blue-500/10 px-4 py-2 text-sm font-medium text-blue-200 transition hover:bg-blue-500/20 disabled:cursor-not-allowed disabled:opacity-60"
            >
              {isSaving ? "Saving…" : "Save Settings"}
            </button>
          </div>
        </form>
      )}
    </section>
  );
};

export default Settings;
