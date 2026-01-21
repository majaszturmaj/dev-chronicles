import React from "react";
import { ActivityLog } from "../../types";
import CollapsibleSection from "./CollapsibleSection";

interface TimelineProps {
  logs: ActivityLog[];
}

const Timeline: React.FC<TimelineProps> = ({ logs }) => {
  return (
    <CollapsibleSection
      title="Activity Timeline"
      count={logs.length}
      countLabel="events"
      maxHeight="500px"
    >
      <ol className="space-y-3">
        {logs.length === 0 && (
          <li className="text-sm text-slate-400 py-4">No events captured yet.</li>
        )}
        {logs.map((log) => (
          <li key={log.id} className="rounded-md border border-slate-800 bg-slate-950/80 p-3">
            <div className="flex items-center justify-between">
              <p className="font-medium text-slate-100">{log.source}</p>
              <time className="text-xs text-slate-400">
                {new Date(log.timestamp).toLocaleString()}
              </time>
            </div>
            <pre className="mt-2 overflow-x-auto text-xs text-slate-300">
              {JSON.stringify(log.payload, null, 2)}
            </pre>
          </li>
        ))}
      </ol>
    </CollapsibleSection>
  );
};

export default Timeline;
