import { createApp } from 'vue'
import App from './App.vue'
import { setupI18n } from './locales'
import { setupAssets, setupScrollbarStyle } from './plugins'
import { setupStore } from './store'
import { setupRouter } from './router'

import { Icon, addCollection } from '@iconify/vue/offline';
import riIcons from '@iconify/json/json/ri.json';

import vuetify from './plugins/vuetify'

async function bootstrap() {
  const app = createApp(App)

  app.use(vuetify)

  app.component('IconifyIcon', Icon);
  // 注册图标
  addCollection(riIcons);

  setupAssets()

  setupScrollbarStyle()

  setupStore(app)

  setupI18n(app)

  await setupRouter(app)

  app.mount('#app')
}

bootstrap()
