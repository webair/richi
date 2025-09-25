import './shared/style/global.css'

import { createApp } from 'vue'

import App from './App.vue'
import { AuthPlugin } from './shared/auth'

createApp(App).use(AuthPlugin).mount('#app')
