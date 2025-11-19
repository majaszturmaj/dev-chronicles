export interface ActivityLog {
  id: number;
  source: string;
  payload: Record<string, unknown>;
  timestamp: string;
}

export interface AiReport {
  id: string;
  summary: string;
  createdAt: string;
}

export interface AiSettings {
  providerUrl: string;
  apiKey?: string | null;
  model_name: string;
}
