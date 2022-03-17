<template>
  <header>{{ msg }}</header>
</template>

<script setup lang="ts">
  const { msg = 'Header' } = defineProps<{
    msg?: String;
  }>();
</script>
