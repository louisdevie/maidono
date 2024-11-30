import { createApp } from 'vue'
import App from './App.vue'
import '/src/assets/app.css'

import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faRocket } from '@fortawesome/free-solid-svg-icons'
library.add(faRocket)

createApp(App).component('fa', FontAwesomeIcon).mount('#app')
