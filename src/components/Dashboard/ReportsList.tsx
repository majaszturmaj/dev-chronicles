import React from "react";
import ReactMarkdown from "react-markdown";
import { AiReport } from "../../types";

interface ReportsListProps {
  reports: AiReport[];
  isLoading?: boolean;
  error?: string;
}

const ReportsList: React.FC<ReportsListProps> = ({ reports, isLoading, error }) => {
  return (
    <div className="space-y-3">
      {isLoading && (
        <p className="text-center text-sm text-slate-400 py-8">Loading reports…</p>
      )}
      
      {error && (
        <p className="text-center text-sm text-red-400 py-4">{error}</p>
      )}
      
      {!isLoading && !error && reports.length === 0 && (
        <p className="text-center text-sm text-slate-400 py-8">
          No reports found for this date.
        </p>
      )}

      {!isLoading && !error && reports.length > 0 && (
        <div className="space-y-3 max-h-[600px] overflow-y-auto pr-2">
          {reports.map((report, index) => (
            <article
              key={report.id}
              className="rounded-lg border border-slate-700 bg-slate-900/50 p-4 hover:border-slate-600 transition"
            >
              <div className="flex items-start justify-between gap-2 mb-3">
                <div className="flex-1">
                  <div className="text-xs text-slate-400">
                    Report #{reports.length - index}
                  </div>
                  <time className="text-xs text-slate-500">
                    {new Date(report.generated_at).toLocaleString()}
                  </time>
                  {report.log_count && (
                    <div className="text-xs text-slate-500 mt-1">
                      {report.log_count} log{report.log_count !== 1 ? "s" : ""} analyzed
                    </div>
                  )}
                  {report.sources && (
                    <div className="text-xs text-slate-500">
                      Sources: {report.sources}
                    </div>
                  )}
                </div>
              </div>

              <div className="prose prose-invert prose-sm max-w-none text-slate-200">
                <ReactMarkdown>{report.summary}</ReactMarkdown>
              </div>
            </article>
          ))}
        </div>
      )}
    </div>
  );
};

export default ReportsList;
