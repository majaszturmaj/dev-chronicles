import React, { useState } from "react";
import Timeline from "./Timeline";
import ReportView from "./ReportView";
import ReportsList from "./ReportsList";
import CollapsibleSection from "./CollapsibleSection";
import { ActivityLog, AiReport } from "../../types";

interface DashboardProps {
  logs: ActivityLog[];
  isLoading: boolean;
  error?: string;
  onGenerateReport: () => void;
  isGeneratingReport: boolean;
  latestReport?: string;
  reportError?: string;
  reportsForDate: AiReport[];
  isLoadingReports: boolean;
  reportsError?: string;
  onDateChange: (date: string) => void;
  selectedDate: string;
}

const Dashboard: React.FC<DashboardProps> = ({
  logs,
  isLoading,
  error,
  onGenerateReport,
  isGeneratingReport,
  latestReport,
  reportError,
  reportsForDate,
  isLoadingReports,
  reportsError,
  onDateChange,
  selectedDate,
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

      <CollapsibleSection
        title="Latest AI Report"
        maxHeight="400px"
        defaultCollapsed={!latestReport}
      >
        <div className="space-y-4">
          <ReportView content={latestReport} />
        </div>
      </CollapsibleSection>

      <CollapsibleSection
        title="All Reports for Date"
        maxHeight="700px"
        defaultCollapsed={reportsForDate.length === 0}
      >
        <div className="space-y-4">
          <div className="flex flex-col gap-2">
            <label htmlFor="reportDatePicker" className="text-sm font-medium text-slate-200">
              Select Date:
            </label>
            <input
              id="reportDatePicker"
              type="date"
              value={selectedDate}
              onChange={(e) => onDateChange(e.target.value)}
              className="rounded-md border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-100 focus:border-blue-500 focus:outline-none"
            />
          </div>
          <ReportsList
            reports={reportsForDate}
            isLoading={isLoadingReports}
            error={reportsError}
          />
        </div>
      </CollapsibleSection>
    </div>
  );
};

export default Dashboard;
