// Styles
import '@mdi/font/css/materialdesignicons.css' 

import 'vuetify/styles'

import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

// Vuetify
import { createVuetify } from 'vuetify'
import { aliases,mdi } from 'vuetify/iconsets/mdi'

export default createVuetify({
  components: { ...components },
  directives,

   icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi,      
    },
  },
}
)
