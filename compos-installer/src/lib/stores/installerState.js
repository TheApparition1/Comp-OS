import { derived, writable } from 'svelte/store';

const initialState = {
  mode: 'simulation',
  userExperienceMode: 'beginner', // 'beginner' or 'experienced'
  currentStep: 0,
  currentTab: 0,
  language: { 
    code: 'en_US',
    region: 'US',
    inputMethod: 'auto',
    dateFormat: 'regional',
    currencyMode: 'regional'
  },
  network: {
    type: 'wired',
    wifiNetwork: '',
    wifiPassword: '',
    enabled: true,
    useDHCP: true,
    staticIP: '',
    staticNetmask: '255.255.255.0',
    staticGateway: '',
    staticDNS: '1.1.1.1,8.8.8.8',
    proxy: '',
    offlineInstall: false
  },
  disk: {
    selectedDisk: '',
    flow: 'guided',
    guidedAction: 'erase_install',
    filesystem: 'ext4',
    swapSizeGb: 4,
    encryptionEnabled: false,
    encryptionPassword: '',
    partitionPlan: [],
    previewSummary: ''
  },
  desktop: {
    selectedDesktop: 'gnome',
    displayManager: 'GDM',
    installExtras: true,
    theme: 'default'
  },
  terminal: {
    selectedTerminal: 'kitty',
    enableFishShell: true
  },
  utilities: {
    selectedApps: [],
    search: '',
    category: 'all'
  },
  browser: {
    selectedBrowser: 'firefox'
  },
  wine: {
    option: 'none'
  },
  user: {
    username: '',
    fullName: '',
    password: '',
    hostname: 'compos-pc',
    autoLogin: false,
    enableSudo: true,
    shell: 'bash'
  },
  datetime: {
    timezone: 'UTC',
    useNTP: true,
    ntpServer: 'pool.ntp.org',
    customNtpServer: '',
    manualTime: '',
    manualDate: '',
    dateFormat: 'regional',
    timeFormat: '24h',
    firstDayOfWeek: 'monday',
    hwclockUtc: true
  },
  installation: {
    mode: 'automated',
    structuredLogs: true,
    retryOnFailure: false
  }
};

export const installerState = writable(initialState);

export const stepValidity = derived(installerState, ($s) => ({
  Welcome: true,
  Language: Boolean($s.language?.code && $s.language?.region),
  Network: $s.network?.type === 'wired' || (Boolean($s.network?.wifiNetwork)),
  Disk: Boolean($s.disk?.selectedDisk),
  Desktop: Boolean($s.desktop?.selectedDesktop),
  Terminal: true,
  Utilities: true,
  Browser: true,
  Wine: true,
  User: Boolean($s.user?.username && $s.user?.fullName && $s.user?.password),
  DateTime: Boolean($s.datetime?.timezone),
  Installation: true
}));

export const getVisibleTabs = (stepName, mode) => {
  const allTabs = {
    Welcome: ['overview'],
    Language: ['language', 'regional', 'input'],
    Network: ['connection', 'wifi', 'advanced', 'proxy'],
    Disk: ['disks', 'partitions', 'advanced', 'encryption'],
    Desktop: ['desktop', 'display', 'extras', 'themes'],
    Terminal: ['main'],
    Utilities: ['main'],
    Browser: ['main'],
    Wine: ['main'],
    User: ['profile', 'security', 'system', 'advanced'],
    DateTime: ['timezone', 'time', 'format', 'advanced'],
    Installation: ['main']
  };

  if (mode === 'experienced') return allTabs[stepName] || ['main'];

  // Beginner mode filters
  const beginnerTabs = {
    Language: ['language', 'regional'],
    Network: ['connection', 'wifi'],
    Disk: ['disks', 'partitions'], // Partitions will be guided-only
    Desktop: ['desktop'],
    User: ['profile', 'security'],
    DateTime: ['timezone']
  };

  return beginnerTabs[stepName] || allTabs[stepName] || ['main'];
};

export const tabValidity = derived(installerState, ($s) => ({
  Welcome: {
    overview: true,
    profiles: true,
    advanced: true
  },
  Language: {
    language: Boolean($s.language?.code),
    regional: Boolean($s.language?.region),
    input: true
  },
  Network: {
    connection: Boolean($s.network?.type),
    wifi: $s.network?.type === 'wireless' ? Boolean($s.network?.wifiNetwork) : true,
    advanced: true,
    proxy: true
  },
  Disk: {
    disks: Boolean($s.disk?.selectedDisk),
    partitions: true,
    advanced: true,
    encryption: true
  },
  Desktop: {
    desktop: Boolean($s.desktop?.selectedDesktop),
    display: true,
    extras: true,
    themes: true
  },
  User: {
    profile: Boolean($s.user?.username && $s.user?.fullName),
    security: Boolean($s.user?.password),
    system: true,
    advanced: true
  },
  DateTime: {
    timezone: Boolean($s.datetime?.timezone),
    time: true,
    format: true,
    advanced: true
  },
  Terminal: { main: true },
  Utilities: { main: true },
  Browser: { main: true },
  Wine: { main: true },
  Installation: { main: true }
}));

export const tabCompletion = derived(installerState, ($s) => ({
  Welcome: { overview: true },
  Language: {
    language: Boolean($s.language?.code),
    regional: Boolean($s.language?.region),
    input: Boolean($s.language?.inputMethod)
  },
  Network: {
    connection: Boolean($s.network?.type),
    wifi: $s.network?.type === 'wireless' ? Boolean($s.network?.wifiNetwork) : true,
    advanced: true,
    proxy: true
  },
  Disk: {
    disks: Boolean($s.disk?.selectedDisk),
    partitions: true,
    advanced: true,
    encryption: true
  },
  Desktop: {
    desktop: Boolean($s.desktop?.selectedDesktop),
    display: true,
    extras: true,
    themes: true
  },
  User: {
    profile: Boolean($s.user?.username && $s.user?.fullName),
    security: Boolean($s.user?.password),
    system: true,
    advanced: true
  },
  DateTime: {
    timezone: Boolean($s.datetime?.timezone),
    time: true,
    format: true,
    advanced: true
  },
  Terminal: { main: true },
  Utilities: { main: true },
  Browser: { main: true },
  Wine: { main: true },
  Installation: { main: true }
}));

export function updateInstallerSection(section, updates) {
  installerState.update((state) => ({
    ...state,
    [section]: {
      ...state[section],
      ...updates
    }
  }));
}

export function setInstallerMode(mode) {
  installerState.update((state) => ({...state, mode}));
}

export function setUserExperienceMode(mode) {
  installerState.update((state) => ({...state, userExperienceMode: mode}));
}

export function setCurrentStep(currentStep) {
  installerState.update((state) => ({...state, currentStep, currentTab: 0}));
}

export function setCurrentTab(currentTab) {
  installerState.update((state) => ({...state, currentTab}));
}
