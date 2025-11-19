import React from "react";
import ReactMarkdown from "react-markdown";

interface ReportViewProps {
  content?: string;
}

const ReportView: React.FC<ReportViewProps> = ({ content }) => {
  return (
    <article className="prose prose-invert max-w-none">
      {content ? (
        <ReactMarkdown>{content}</ReactMarkdown>
      ) : (
        <p className="text-slate-400">Summaries will appear here once generated.</p>
      )}
    </article>
  );
};

export default ReportView;
