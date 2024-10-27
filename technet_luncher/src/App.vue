<script setup lang="ts">
import GamesPage from './components/GamesPage.vue';
import SideBar from './components/SideBar.vue';

</script>

<template export>
  <v-app>
    <v-card>
      <v-layout>
        <SideBar/>

        <GamesPage/>
      </v-layout>
    </v-card>
  </v-app>

</template>

<style>
html {
  overflow: hidden;
}
</style>
