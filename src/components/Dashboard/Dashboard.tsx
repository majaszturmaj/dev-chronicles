import React from "react";
import Timeline from "./Timeline";
import ReportView from "./ReportView";
import { ActivityLog } from "../../types";

interface DashboardProps {
  logs: ActivityLog[];
  isLoading: boolean;
  error?: string;
  onGenerateReport: () => void;
  isGeneratingReport: boolean;
  latestReport?: string;
  reportError?: string;
}

const Dashboard: React.FC<DashboardProps> = ({
  logs,
  isLoading,
  error,
  onGenerateReport,
  isGeneratingReport,
  latestReport,
  reportError,
}) => {
  return (
    <div className="space-y-6">
      <section className="rounded-lg border border-slate-800 bg-slate-900/60 p-4">
        <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
          <div>
            <h1 className="text-2xl font-bold">DevChronicle Dashboard</h1>
            <p className="mt-2 text-sm text-slate-400">
              Debug view: fetched <span className="font-semibold text-slate-200">{logs.length}</span> logs
              for the selected date.
            </p>
          </div>
          <button
            type="button"
            onClick={onGenerateReport}
            disabled={isGeneratingReport}
            className="inline-flex items-center justify-center rounded-md border border-blue-500 bg-blue-500/10 px-4 py-2 text-sm font-medium text-blue-200 transition hover:bg-blue-500/20 disabled:cursor-not-allowed disabled:opacity-60"
          >
            {isGeneratingReport ? "Generating…" : "Generate Report Now"}
          </button>
        </div>
        {isLoading && <p className="mt-2 text-sm text-slate-400">Loading logs…</p>}
        {error && (
          <p className="mt-2 text-sm text-red-400" role="alert">
            {error}
          </p>
        )}
        {reportError && (
          <p className="mt-2 text-sm text-red-400" role="alert">
            {reportError}
          </p>
        )}
      </section>

      <Timeline logs={logs} />

      <section className="space-y-4 rounded-lg border border-slate-800 bg-slate-900/60 p-4">
        <h2 className="text-xl font-semibold">AI Reports</h2>
        <ReportView content={latestReport} />
      </section>
    </div>
  );
};

export default Dashboard;
