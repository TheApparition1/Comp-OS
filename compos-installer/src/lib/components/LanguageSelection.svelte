<script>
  import { Globe, MapPin, Keyboard, Clock3, DollarSign, ChevronRight, Search } from 'lucide-svelte';
  import { installerState, updateInstallerSection, setCurrentTab } from '$lib/stores/installerState.js';
  
  $: activeTab = tabs[$installerState.currentTab] || 'language';
  
  let selectedLanguage = 'en_US';
  let selectedRegion = 'US';
  let searchQuery = '';
  
  const tabs = ['language', 'regional', 'input'];
  const tabLabels = {
    language: 'Language',
    regional: 'Regional',
    input: 'Input'
  };
  const tabIcons = {
    language: Globe,
    regional: MapPin,
    input: Keyboard
  };
  let keyboardLayout = 'auto';
  let dateFormat = 'regional';
  let currencyMode = 'regional';
  
  let languages = [
    { code: 'en_US', name: 'English (United States)', region: 'US', nativeName: 'English (United States)' },
    { code: 'en_GB', name: 'English (United Kingdom)', region: 'GB', nativeName: 'English (United Kingdom)' },
    { code: 'en_CA', name: 'English (Canada)', region: 'CA', nativeName: 'English (Canada)' },
    { code: 'en_AU', name: 'English (Australia)', region: 'AU', nativeName: 'English (Australia)' },
    { code: 'es_ES', name: 'Spanish (Spain)', region: 'ES', nativeName: 'Español (España)' },
    { code: 'es_MX', name: 'Spanish (Mexico)', region: 'MX', nativeName: 'Español (México)' },
    { code: 'fr_FR', name: 'French (France)', region: 'FR', nativeName: 'Français (France)' },
    { code: 'fr_CA', name: 'French (Canada)', region: 'CA', nativeName: 'Français (Canada)' },
    { code: 'de_DE', name: 'German (Germany)', region: 'DE', nativeName: 'Deutsch (Deutschland)' },
    { code: 'de_AT', name: 'German (Austria)', region: 'AT', nativeName: 'Deutsch (Österreich)' },
    { code: 'it_IT', name: 'Italian (Italy)', region: 'IT', nativeName: 'Italiano (Italia)' },
    { code: 'pt_BR', name: 'Portuguese (Brazil)', region: 'BR', nativeName: 'Português (Brasil)' },
    { code: 'pt_PT', name: 'Portuguese (Portugal)', region: 'PT', nativeName: 'Português (Portugal)' },
    { code: 'ru_RU', name: 'Russian (Russia)', region: 'RU', nativeName: 'Русский (Россия)' },
    { code: 'ja_JP', name: 'Japanese (Japan)', region: 'JP', nativeName: '日本語 (日本)' },
    { code: 'zh_CN', name: 'Chinese (Simplified)', region: 'CN', nativeName: '简体中文 (中国)' },
    { code: 'zh_TW', name: 'Chinese (Traditional)', region: 'TW', nativeName: '繁體中文 (台灣)' },
    { code: 'ko_KR', name: 'Korean (South Korea)', region: 'KR', nativeName: '한국어 (대한민국)' },
    { code: 'ar_SA', name: 'Arabic (Saudi Arabia)', region: 'SA', nativeName: 'العربية (المملكة العربية السعودية)' },
    { code: 'hi_IN', name: 'Hindi (India)', region: 'IN', nativeName: 'हिन्दी (भारत)' },
    { code: 'nl_NL', name: 'Dutch (Netherlands)', region: 'NL', nativeName: 'Nederlands (Nederland)' },
    { code: 'pl_PL', name: 'Polish (Poland)', region: 'PL', nativeName: 'Polski (Polska)' },
    { code: 'tr_TR', name: 'Turkish (Turkey)', region: 'TR', nativeName: 'Türkçe (Türkiye)' },
    { code: 'sv_SE', name: 'Swedish (Sweden)', region: 'SE', nativeName: 'Svenska (Sverige)' },
    { code: 'no_NO', name: 'Norwegian (Norway)', region: 'NO', nativeName: 'Norsk (Norge)' },
    { code: 'da_DK', name: 'Danish (Denmark)', region: 'DK', nativeName: 'Dansk (Danmark)' },
    { code: 'fi_FI', name: 'Finnish (Finland)', region: 'FI', nativeName: 'Suomi (Suomi)' },
    { code: 'el_GR', name: 'Greek (Greece)', region: 'GR', nativeName: 'Ελληνικά (Ελλάδα)' },
    { code: 'he_IL', name: 'Hebrew (Israel)', region: 'IL', nativeName: 'עברית (ישראל)' },
    { code: 'th_TH', name: 'Thai (Thailand)', region: 'TH', nativeName: 'ไทย (ประเทศไทย)' },
    { code: 'vi_VN', name: 'Vietnamese (Vietnam)', region: 'VN', nativeName: 'Tiếng Việt (Việt Nam)' },
    { code: 'id_ID', name: 'Indonesian (Indonesia)', region: 'ID', nativeName: 'Bahasa Indonesia (Indonesia)' },
    { code: 'ms_MY', name: 'Malay (Malaysia)', region: 'MY', nativeName: 'Bahasa Melayu (Malaysia)' },
    { code: 'cs_CZ', name: 'Czech (Czech Republic)', region: 'CZ', nativeName: 'Čeština (Česká republika)' },
    { code: 'sk_SK', name: 'Slovak (Slovakia)', region: 'SK', nativeName: 'Slovenčina (Slovensko)' },
    { code: 'hu_HU', name: 'Hungarian (Hungary)', region: 'HU', nativeName: 'Magyar (Magyarország)' },
    { code: 'ro_RO', name: 'Romanian (Romania)', region: 'RO', nativeName: 'Română (România)' },
    { code: 'bg_BG', name: 'Bulgarian (Bulgaria)', region: 'BG', nativeName: 'Български (България)' },
    { code: 'hr_HR', name: 'Croatian (Croatia)', region: 'HR', nativeName: 'Hrvatski (Hrvatska)' },
    { code: 'sr_RS', name: 'Serbian (Serbia)', region: 'RS', nativeName: 'Српски (Србија)' },
    { code: 'uk_UA', name: 'Ukrainian (Ukraine)', region: 'UA', nativeName: 'Українська (Україна)' },
    { code: 'et_EE', name: 'Estonian (Estonia)', region: 'EE', nativeName: 'Eesti (Eesti)' },
    { code: 'lv_LV', name: 'Latvian (Latvia)', region: 'LV', nativeName: 'Latviešu (Latvija)' },
    { code: 'lt_LT', name: 'Lithuanian (Lithuania)', region: 'LT', nativeName: 'Lietuvių (Lietuva)' },
    { code: 'sl_SI', name: 'Slovenian (Slovenia)', region: 'SI', nativeName: 'Slovenščina (Slovenija)' }
  ];

  function selectLanguage(langCode) {
    selectedLanguage = langCode;
    const lang = languages.find(l => l.code === langCode);
    if (lang) {
      selectedRegion = lang.region;
    }
  }

  $: filteredLanguages = languages.filter(lang => 
    lang.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    lang.nativeName.toLowerCase().includes(searchQuery.toLowerCase()) ||
    lang.code.toLowerCase().includes(searchQuery.toLowerCase())
  );

  $: {
    updateInstallerSection('language', {
      code: selectedLanguage,
      region: selectedRegion,
      inputMethod: keyboardLayout,
      dateFormat,
      currencyMode
    });
  }
</script>

<div class="max-w-4xl mx-auto">
  <div class="mb-10">
    <h2 class="text-2xl font-bold text-slate-900 mb-2">Language and Regionalization</h2>
    <p class="text-slate-500">Configure your preferred system language, input methods, and regional standards.</p>
  </div>

  <div class="flex gap-1 mb-8 bg-slate-100 p-1 rounded-xl w-fit">
    {#each tabs as tab}
      <button 
        class="px-5 py-2 rounded-lg text-sm font-bold transition-all flex items-center gap-2 {activeTab === tab ? 'bg-white text-blue-600 shadow-sm' : 'text-slate-500 hover:text-slate-700'}"
        on:click={() => setCurrentTab(tabs.indexOf(tab))}
      >
        <svelte:component this={tabIcons[tab]} class="w-4 h-4" />
        {tabLabels[tab]}
      </button>
    {/each}
  </div>

  <div class="bg-white border border-slate-200 rounded-2xl p-8 shadow-sm min-h-[400px]">
    {#if activeTab === 'language'}
      <div class="space-y-6">
        <div class="relative">
          <Search class="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-slate-300" />
          <input 
            type="text" 
            placeholder="Search system languages..." 
            class="w-full pl-12 pr-4 py-4 bg-slate-50 border border-slate-100 rounded-2xl focus:outline-none focus:ring-2 focus:ring-blue-600/10 focus:border-blue-600 transition-all font-medium text-slate-700"
            bind:value={searchQuery}
          />
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-3 max-h-[300px] overflow-y-auto pr-2 scrollbar-hide">
          {#each filteredLanguages as lang}
            <button 
              class="flex items-center justify-between p-4 border rounded-xl transition-all {selectedLanguage === lang.code ? 'border-blue-600 bg-blue-50/50' : 'border-slate-100 hover:border-slate-200'}"
              on:click={() => selectLanguage(lang.code)}
            >
              <div class="text-left">
                <p class="font-bold text-sm text-slate-900">{lang.nativeName}</p>
                <p class="text-[10px] text-slate-500 uppercase tracking-widest font-bold">{lang.name}</p>
              </div>
              {#if selectedLanguage === lang.code}
                <div class="w-6 h-6 rounded-full bg-blue-600 flex items-center justify-center text-white">
                  <span class="text-[10px] font-bold">✓</span>
                </div>
              {/if}
            </button>
          {/each}
        </div>
      </div>

    {:else if activeTab === 'regional'}
      <div class="space-y-8">
        <h3 class="text-lg font-bold text-slate-900 mb-4">Regional Standards</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="space-y-3">
            <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest" for="date-fmt">Date & Time Format</label>
            <select id="date-fmt" class="w-full p-4 bg-slate-50 border border-slate-100 rounded-xl focus:border-blue-600 focus:outline-none appearance-none font-medium text-slate-700" bind:value={dateFormat}>
              <option value="regional">Regional Default</option>
              <option value="iso">ISO 8601 (2024-05-04)</option>
              <option value="us">United States (05/04/2024)</option>
              <option value="eu">Europe (04/05/2024)</option>
            </select>
          </div>

          <div class="space-y-3">
            <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest" for="curr-fmt">Currency Standards</label>
            <select id="curr-fmt" class="w-full p-4 bg-slate-50 border border-slate-100 rounded-xl focus:border-blue-600 focus:outline-none appearance-none font-medium text-slate-700" bind:value={currencyMode}>
              <option value="regional">Regional Default</option>
              <option value="usd">USD Standards</option>
              <option value="eur">EUR Standards</option>
            </select>
          </div>
        </div>
      </div>

    {:else if activeTab === 'input'}
      <div class="space-y-6">
        <h3 class="text-lg font-bold text-slate-900 mb-4">Keyboard & Input</h3>
        <div class="space-y-4">
          <label class="block text-[10px] font-bold text-slate-400 uppercase tracking-widest" for="kb-layout">Keyboard Layout</label>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
            {#each ['auto', 'us', 'uk', 'de', 'fr', 'es'] as layout}
              <button 
                class="p-4 border-2 rounded-xl font-bold text-sm transition-all {keyboardLayout === layout ? 'border-blue-600 bg-blue-50/50 text-blue-600' : 'border-slate-100 text-slate-400 hover:border-slate-200'}"
                on:click={() => keyboardLayout = layout}
              >
                {layout.toUpperCase()}
              </button>
            {/each}
          </div>
        </div>
      </div>
    {/if}
  </div>

  <div class="mt-8 flex items-center justify-between p-6 bg-slate-50 rounded-2xl border border-slate-100">
    <div class="flex items-center gap-4">
      <div class="w-10 h-10 rounded-full bg-blue-100 text-blue-600 flex items-center justify-center">
        <MapPin class="w-5 h-5" />
      </div>
      <div>
        <p class="font-bold text-slate-900 text-sm">Target Region</p>
        <p class="text-xs text-slate-500">System region will be set to <span class="text-blue-600 font-bold">{selectedRegion}</span></p>
      </div>
    </div>
  </div>
</div>
