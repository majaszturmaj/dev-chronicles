export interface ActivityLog {
  id: number;
  source: string;
  payload: Record<string, unknown>;
  timestamp: string;
}

export interface AiReport {
  id: string | number;
  summary: string;
  generated_at: string;
  log_count?: number;
  sources?: string;
  session_id?: string;
}

export interface AiSettings {
  providerUrl: string;
  apiKey?: string | null;
  model_name: string;
}
