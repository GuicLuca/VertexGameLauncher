<script setup lang="ts">
</script>

<template>
  <v-navigation-drawer expand-on-hover rail rail-width="60">
    <v-list>
      <v-list-item prepend-avatar="../../src-tauri/icons/icon.png" subtitle="v0.0.1"
        title="Vertex Launcher" class=""></v-list-item>
    </v-list>

    <v-divider></v-divider>

    <v-list density="compact" nav>
      <!-- <v-list-item
        v-for="game in props.gamelist"
        :prepend-icon="game.navigation_icon"
        :title="game.title"
        :value="game.title"
      ></v-list-item> -->
      <v-list-item prepend-icon="mdi-folder" title="Game 1" value="g1"></v-list-item>
      <v-list-item prepend-icon="mdi-account-multiple" title="Game 2" value="g2"></v-list-item>
      <v-list-item prepend-icon="mdi-star" title="Game 3" value="g3"></v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<style scoped>
</style>