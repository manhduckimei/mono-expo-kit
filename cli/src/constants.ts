import { CliResults } from './type'

export const TITLE_TEXT = 'create expo kit'

export const DEFAULT_APP_NAME = 'my-expo-kit'

export const defaultOptions: CliResults = {
  projectName: DEFAULT_APP_NAME,
  packages: [],
  flags: {
    noGit: false,
    noInstall: false,
    overwrite: false,
    importAlias: '~/',
    packageManager: undefined,
    eas: false
  }
}
