import { ss } from '@/utils/storage'

const LOCAL_NAME = 'settingsStorage'

export interface SettingsState {
  systemMessage: string,
  multiRoundChat: number,
  maxTokens: number,
  temperature: number
  top_p: number,

  previewLanguages: string[],
  exportFormat: string[],

}

export function defaultSetting(): SettingsState {
  return {
    systemMessage: 'You are DeepSeek, a large language model. Follow the user\'s instructions carefully. Respond using markdown.',
    multiRoundChat: 5,
    maxTokens: 4096,
    temperature: 0.6,
    top_p: 1,

    previewLanguages: ['markdown', 'mermaid', 'html', 'python', 'dot'],
    exportFormat: ['md', 'html', 'docx', 'pptx', 'latex']

  }
}

export function getLocalState(): SettingsState {
  const localSetting: SettingsState | undefined = ss.get(LOCAL_NAME)
  return { ...defaultSetting(), ...localSetting }
}

export function setLocalState(setting: SettingsState): void {
  ss.set(LOCAL_NAME, setting)
}

export function removeLocalState() {
  ss.remove(LOCAL_NAME)
}
