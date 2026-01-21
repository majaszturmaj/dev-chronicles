import React, { useState, ReactNode } from "react";

interface CollapsibleSectionProps {
  title: string;
  children: ReactNode;
  defaultCollapsed?: boolean;
  maxHeight?: string;
  count?: number;
  countLabel?: string;
}

const CollapsibleSection: React.FC<CollapsibleSectionProps> = ({
  title,
  children,
  defaultCollapsed = false,
  maxHeight = "400px",
  count,
  countLabel,
}) => {
  const [isCollapsed, setIsCollapsed] = useState(defaultCollapsed);

  return (
    <section className="rounded-lg border border-slate-800 bg-slate-900/60 overflow-hidden">
      <header 
        className="flex items-center justify-between p-4 cursor-pointer hover:bg-slate-900/80 transition"
        onClick={() => setIsCollapsed(!isCollapsed)}
      >
        <div className="flex items-center gap-2">
          <h2 className="text-xl font-semibold">{title}</h2>
          {count !== undefined && (
            <span className="text-sm text-slate-400">
              {count} {countLabel || "items"}
            </span>
          )}
        </div>
        <button
          type="button"
          className="text-slate-400 hover:text-slate-200 transition"
          aria-label={isCollapsed ? "Expand" : "Collapse"}
        >
          <svg
            className={`w-5 h-5 transition-transform ${isCollapsed ? "" : "rotate-180"}`}
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M19 9l-7 7-7-7"
            />
          </svg>
        </button>
      </header>
      {!isCollapsed && (
        <div
          className="overflow-y-auto px-4 pb-4"
          style={{ maxHeight }}
        >
          {children}
        </div>
      )}
    </section>
  );
};

export default CollapsibleSection;

