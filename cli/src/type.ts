// export types
export interface CliFlags {
  noGit: boolean
  noInstall: boolean
  overwrite: boolean
  importAlias: string | boolean
  packageManager: PackageManager | undefined
  eas: boolean
}

export const availablePackages = [
  'react-query',
  'flashlist',
  'dayjs',
  'axios',
  'hookform',
  '@react-navigation/drawer',
  'react-navigation',
  'react-native-gesture-handler',
  'react-native-reanimated',
  'svg',
  'stylesheet',
  'restyle',
  'unistyles',
  'i18next'
] as const

export type NavigationTypes = 'stack' | 'tabs' | 'drawer + tabs' | undefined

export type StylingSelect = 'restyle' | 'stylesheet' | 'unistyles'

export type PackageManager = 'yarn' | 'npm' | 'pnpm' | 'bun'

export type Internalization = 'i18next'

export type AvailablePackages = {
  name: (typeof availablePackages)[number]
  type: 'navigation' | 'styling' | 'internationalization' | 'utils'
  options?: {
    type?: NavigationTypes
  }
}

export interface CliResults {
  projectName: string
  packages: AvailablePackages[]
  flags: CliFlags
}
