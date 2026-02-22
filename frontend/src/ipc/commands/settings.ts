import { invoke } from '../invoke';
import type { AppConfig } from '../types/settings';

export async function getConfig(): Promise<AppConfig> {
  return invoke<AppConfig>('get_config');
}

export async function updateConfig(config: Partial<AppConfig>): Promise<void> {
  return invoke<void>('update_config', { config });
}

export async function getDataDir(): Promise<string> {
  return invoke<string>('get_data_dir');
}
